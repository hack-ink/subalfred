//! Full functionality JSONRPC websocket client implementation.
//! Follow https://www.jsonrpc.org/specification specification.

// std
use std::{collections::HashMap, str, sync::Arc, time::Duration};
// crates.io
use futures::{
	future::{self, Either::*, Fuse},
	stream, FutureExt, SinkExt, StreamExt,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::{
	sync::{mpsc, oneshot},
	time,
};
use tokio_stream::wrappers::IntervalStream;
use tokio_tungstenite::tungstenite::{error::Result as WsResult, Message};
// hack-ink
use super::*;
use crate::core::{error, Result};
use subrpcer::system;

type Messenger = mpsc::Sender<Call>;

type RequestResponse = Response<Value>;
type RequestNotifier = oneshot::Sender<RequestResponse>;
type RequestPool = HashMap<Id, RequestNotifier>;

type BatchResponse = Vec<RequestResponse>;
type BatchNotifier = oneshot::Sender<BatchResponse>;
type BatchPool = HashMap<Id, BatchNotifier>;

/// The websocket builder.
///
/// Connect this to acquire a websocket object.
#[derive(Debug)]
pub struct Client {
	concurrency_limit: Id,
	interval: Duration,
	request_timeout: Duration,
}
impl Client {
	/// Connect to the given URI.
	pub async fn connect(self, uri: &str) -> Result<Websocket> {
		let (mut ws_tx, mut ws_rx) = tokio_tungstenite::connect_async(uri)
			.await
			.map_err(error::Generic::Tungstenite)?
			.0
			.split();
		let (tx, rx) = mpsc::channel(1);

		tokio::spawn(async move {
			let system_health_req = serde_json::to_string(&system::health_once()).unwrap();
			let rx = stream::unfold(rx, |mut r| async { r.recv().await.map(|c| (c, r)) });

			futures::pin_mut!(rx);

			#[cfg(feature = "debug-websocket")]
			{
				let mut rx = rx;

				loop {
					match rx.next().await.unwrap() {
						call @ Call::Debug(_) => tracing::info!("{call:?}"),
						_ => unreachable!(),
					}
				}
			}

			// TODO: clean dead items?
			let mut pool = Pool::new();
			let mut rxs_fut = future::select(rx.next(), ws_rx.next());
			// Minimum interval is 1ms.
			let interval = self.interval.max(Duration::from_millis(1));
			let mut interval = IntervalStream::new(time::interval(interval));
			// Disable the tick, if the interval is zero.
			let mut tick_fut =
				if self.interval.is_zero() { Fuse::terminated() } else { interval.next().fuse() };

			loop {
				// tokio::select! {
				// 	_ = tick => {
				// 		tracing::debug!("Tick(system_health)");
				//
				// 		ws_tx.send(Message::Text(system_health_req.clone())).await.unwrap();
				// 	},
				// 	maybe_recv = ws_rx.next() => pool.on_ws_recv(maybe_recv).await.unwrap(),
				// 	msg = rx.recv() => {
				// 		if let Some(msg) = msg {
				// 			match msg {
				// 				Call::Single(RawCall { id, request, notifier }) => {
				// 					tracing::debug!("{request}");
				//
				// 					ws_tx.send(Message::Text(request)).await.unwrap();
				// 					pool.requests.insert(id, notifier);
				// 				}
				// 				Call::Batch(RawCall { id, request, notifier }) => {
				// 					tracing::debug!("{request}");
				//
				// 					ws_tx.send(Message::Text(request)).await.unwrap();
				// 					pool.batches.insert(id, notifier);
				// 				}
				// 			}
				// 		}
				// 	}
				// }

				match future::select(rxs_fut, tick_fut).await {
					Left((Left((maybe_call, maybe_resp_fut)), tick_fut_)) => {
						if let Some(call) = maybe_call {
							match call {
								#[cfg(feature = "debug-websocket")]
								Call::Debug(_) => {
									tracing::info!("{call:?}");
								},
								Call::Single(RawCall { id, request, notifier }) => {
									tracing::debug!("{request}");

									ws_tx.send(Message::Text(request)).await.unwrap();
									pool.requests.insert(id, notifier);
								},
								Call::Batch(RawCall { id, request, notifier }) => {
									tracing::debug!("{request}");

									ws_tx.send(Message::Text(request)).await.unwrap();
									pool.batches.insert(id, notifier);
								},
							}
						} else {
							//
						}

						rxs_fut = future::select(rx.next(), maybe_resp_fut);
						tick_fut = tick_fut_;
					},
					Left((Right((maybe_resp, maybe_call_fut)), tick_fut_)) => {
						if let Some(resp) = maybe_resp {
							pool.on_ws_recv(resp).await.unwrap()
						} else {
							//
						}

						rxs_fut = future::select(maybe_call_fut, ws_rx.next());
						tick_fut = tick_fut_;
					},
					Right((_, rxs_fut_)) => {
						tracing::debug!("Tick(system_health)");

						ws_tx.send(Message::Text(system_health_req.clone())).await.unwrap();

						rxs_fut = rxs_fut_;
						tick_fut = interval.next().fuse();
					},
				}
			}
		});

		Ok(Websocket {
			messenger: tx,
			request_queue: RequestQueue {
				size: self.concurrency_limit,
				active: Arc::new(()),
				// Id 0 is reserved for system health check.
				next: AtomicUsize::new(1),
			},
			request_timeout: self.request_timeout,
		})
	}
}
impl Default for Client {
	fn default() -> Self {
		Self {
			concurrency_limit: 512,
			interval: Duration::from_secs(0),
			request_timeout: Duration::from_secs(5),
		}
	}
}
/// Connect to the given URI with default configurations.
pub async fn connect(uri: &str) -> Result<Websocket> {
	Client::default().connect(uri).await
}

/// Websocket instance.
///
/// Use this to interact with the server.
pub struct Websocket {
	messenger: Messenger,
	request_queue: RequestQueue,
	request_timeout: Duration,
}
impl Websocket {
	const VERSION: &'static str = "2.0";

	/// Send a single request.
	pub async fn request<'a, D, R>(&self, raw_request: R) -> Result<Response<D>>
	where
		D: DeserializeOwned,
		R: Into<RawRequest<'a, Value>>,
	{
		#[cfg(feature = "debug-websocket")]
		{
			for i in 0..110 {
				// let r = self.messenger.try_send(Call::Debug(i));
				// tracing::trace!("{r:?}");
				self.messenger.send(Call::Debug(i)).await.unwrap();
			}
		}

		let RequestQueueGuard { lock: id, .. } = self.request_queue.next()?;
		let RawRequest { method, params } = raw_request.into();
		let (tx, rx) = oneshot::channel();

		self.messenger
			.send(Call::Single(RawCall {
				id,
				request: serde_json::to_string(&Request {
					jsonrpc: Self::VERSION,
					id,
					method,
					params,
				})
				.map_err(error::Generic::Serde)?,
				notifier: tx,
			}))
			.await
			.map_err(|_| error::Tokio::MpscSend)?;

		let response = time::timeout(self.request_timeout, rx)
			.await
			.map_err(error::Tokio::Elapsed)?
			.map_err(error::Tokio::OneshotRecv)?;

		Ok(Response {
			jsonrpc: response.jsonrpc,
			id: response.id,
			result: serde_json::from_value(response.result).map_err(error::Generic::Serde)?,
		})
	}

	/// Send a batch of requests.
	pub async fn batch<'a, D, R>(&self, raw_requests: Vec<R>) -> Result<Vec<Response<D>>>
	where
		D: DeserializeOwned,
		R: Into<RawRequest<'a, Value>>,
	{
		if raw_requests.is_empty() {
			Err(error::Jsonrpc::EmptyBatch)?;
		}

		let RequestQueueGuard { lock: ids, .. } = self.request_queue.take(raw_requests.len())?;
		let id = ids
			.first()
			.ok_or(error::Generic::AlmostImpossible(E_REQUEST_QUEUE_GUARD_BROKE))?
			.to_owned();
		let requests = ids
			.into_iter()
			.zip(raw_requests.into_iter())
			.map(|(id, raw_req)| {
				let RawRequest { method, params } = raw_req.into();

				Request { jsonrpc: Self::VERSION, id, method, params }
			})
			.collect::<Vec<_>>();
		let request = serde_json::to_string(&requests).map_err(error::Generic::Serde)?;
		let (tx, rx) = oneshot::channel();

		self.messenger
			.send(Call::Batch(RawCall { id, request, notifier: tx }))
			.await
			.map_err(|_| error::Tokio::MpscSend)?;

		let mut responses = rx
			.await
			.map_err(error::Tokio::OneshotRecv)?
			.into_iter()
			.map(|r| {
				Ok(Response {
					jsonrpc: r.jsonrpc,
					id: r.id,
					result: serde_json::from_value(r.result).map_err(error::Generic::Serde)?,
				})
			})
			.collect::<Result<Vec<_>>>()?;

		// Each id is unique.
		responses.sort_unstable_by_key(|r| r.id);

		Ok(responses)
	}
}

#[derive(Debug, Default)]
struct Pool {
	requests: RequestPool,
	batches: BatchPool,
}
impl Pool {
	fn new() -> Self {
		Default::default()
	}

	async fn on_ws_recv(&mut self, response: WsResult<Message>) -> Result<()> {
		match response {
			Ok(msg) => {
				match msg {
					Message::Binary(raw_resp) =>
						self.process_raw_response(str::from_utf8(&raw_resp).unwrap()).await,
					Message::Text(raw_resp) => self.process_raw_response(&raw_resp).await,
					Message::Ping(_) => tracing::debug!("Ping"),
					Message::Pong(_) => tracing::debug!("Pong"),
					Message::Close(_) => unimplemented!("Close"),
					Message::Frame(_) => unimplemented!("Frame"),
				}

				Ok(())
			},
			Err(e) => Err(error::Generic::Tungstenite(e))?,
		}
	}

	// TODO: error handling
	async fn process_raw_response(&mut self, raw_response: &str) {
		if let Ok(response) = serde_json::from_str::<RequestResponse>(raw_response) {
			if response.id == 0 {
				tracing::debug!("Tick({raw_response})");

				return;
			}

			let notifier = self.requests.remove(&response.id).unwrap();

			if let Err(e) = notifier.send(response) {
				tracing::error!("{e:?}");
			}
		} else if let Ok(responses) = serde_json::from_str::<BatchResponse>(raw_response) {
			let notifier = self.batches.remove(&responses.first().unwrap().id).unwrap();

			if let Err(e) = notifier.send(responses) {
				tracing::error!("{e:?}");
			}
		} else {
			tracing::error!("unable to process raw message");
		}
	}
}

#[derive(Debug)]
enum Call {
	#[cfg(feature = "debug-websocket")]
	Debug(u128),
	Single(RawCall<RequestNotifier>),
	Batch(RawCall<BatchNotifier>),
}

/// A single request object.
/// `id`: Request Id.
//
/// Or
///
/// A batch requests object to send several request objects simultaneously.
/// `id`: The first request's id.
#[derive(Debug)]
struct RawCall<N> {
	id: Id,
	request: String,
	notifier: N,
}
