//! Full functionality WS JSONRPC client implementation.
//! Follow <https://www.jsonrpc.org/specification> specification.

// std
use std::{str, sync::Arc, time::Duration};
// crates.io
use futures::{future::Fuse, FutureExt, SinkExt, StreamExt};
// #[cfg(feature = "futures-selector")] use futures::{
// 	future::{self, Either::*},
// 	stream,
// };
use fxhash::FxHashMap;
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
use crate::prelude::*;
use subrpcer::system;

type Messenger = mpsc::Sender<Call>;

type RequestResponse = Response<Value>;
type RequestNotifier = oneshot::Sender<RequestResponse>;
type RequestPool = FxHashMap<Id, RequestNotifier>;

type BatchResponse = Vec<RequestResponse>;
type BatchNotifier = oneshot::Sender<BatchResponse>;
type BatchPool = FxHashMap<Id, BatchNotifier>;

/// The WS initializer.
#[derive(Debug)]
pub struct Initializer {
	/// Concurrent tasks count limit.
	pub concurrency_limit: Id,
	/// Tick interval.
	pub interval: Duration,
	/// Request timeout.
	pub request_timeout: Duration,
}
impl Initializer {
	/// Create a default initializer.
	pub fn new() -> Self {
		Default::default()
	}

	/// Set the [`concurrency_limit`](#structfield.concurrency_limit).
	pub fn concurrency_limit(mut self, concurrency_limit: Id) -> Self {
		self.concurrency_limit = concurrency_limit;

		self
	}

	/// Set the [`interval`](#structfield.interval).
	pub fn interval(mut self, interval: Duration) -> Self {
		self.interval = interval;

		self
	}

	/// Set the [`request_timeout`](#structfield.request_timeout).
	pub fn request_timeout(mut self, request_timeout: Duration) -> Self {
		self.request_timeout = request_timeout;

		self
	}

	/// Initialize the WS stream.
	pub async fn connect(self, uri: &str) -> Result<Ws> {
		let (mut ws_tx, mut ws_rx) = tokio_tungstenite::connect_async(uri)
			.await
			.map_err(error::Generic::Tungstenite)?
			.0
			.split();
		let (tx, rx) = mpsc::channel(self.concurrency_limit);

		tokio::spawn(async move {
			let system_health_request = serde_json::to_string(&system::health(0)).unwrap();
			let mut rx = rx;
			// TODO: clean dead items?
			let mut pool = Pool::new();
			// Minimum interval is 1ms.
			let interval = self.interval.max(Duration::from_millis(1));
			let mut interval = IntervalStream::new(time::interval(interval));
			// Disable the tick, if the interval is zero.
			let mut interval_fut =
				if self.interval.is_zero() { Fuse::terminated() } else { interval.next().fuse() };

			loop {
				tokio::select! {
					_ = &mut interval_fut => {
						tracing::trace!("TickRequest({system_health_request})");

						ws_tx.send(Message::Text(system_health_request.clone())).await.unwrap();

						interval_fut = interval.next().fuse();
					},
					maybe_call = rx.recv() => {
						if let Some(call) = maybe_call {
							match call {
								// Debug.
								// Call::Debug(_) => {
								// 	tracing::debug!("{call:?}");
								// }
								Call::Single(RawCall { id, request, notifier }) => {
									tracing::trace!("SingleRequest({request})");

									ws_tx.send(Message::Text(request)).await.unwrap();
									pool.requests.insert(id, notifier);
								}
								Call::Batch(RawCall { id, request, notifier }) => {
									tracing::trace!("BatchRequests({request})");

									ws_tx.send(Message::Text(request)).await.unwrap();
									pool.batches.insert(id, notifier);
								}
							}
						} else {
							//
						}
					},
					maybe_response = ws_rx.next() => {
						if let Some(response) = maybe_response {
							pool.on_ws_recv(response).await.unwrap()
						} else {
							//
						}
					}
				}
			}
		});

		// TODO: move to another function
		// #[cfg(feature = "futures-selector")]
		// tokio::spawn(async move {
		// 	let system_health_request = serde_json::to_string(&system::health_once()).unwrap();
		// 	let rx = stream::unfold(rx, |mut r| async { r.recv().await.map(|c| (c, r)) });

		// 	futures::pin_mut!(rx);

		// 	let mut rxs_fut = future::select(rx.next(), ws_rx.next());

		// 	// TODO: clean dead items?
		// 	let mut pool = Pool::new();
		// 	// Minimum interval is 1ms.
		// 	let interval = self.interval.max(Duration::from_millis(1));
		// 	let mut interval = IntervalStream::new(time::interval(interval));
		// 	// Disable the tick, if the interval is zero.
		// 	let mut interval_fut =
		// 		if self.interval.is_zero() { Fuse::terminated() } else { interval.next().fuse() };

		// 	loop {
		// 		match future::select(rxs_fut, interval_fut).await {
		// 			Left((Left((maybe_call, maybe_response_fut)), interval_fut_)) => {
		// 				if let Some(call) = maybe_call {
		// 					match call {
		// 						// Debug.
		// 						// Call::Debug(_) => {
		// 						// 	tracing::debug!("{call:?}");
		// 						// },
		// 						Call::Single(RawCall { id, request, notifier }) => {
		// 							tracing::trace!("SingleRequest({request})");

		// 							ws_tx.send(Message::Text(request)).await.unwrap();
		// 							pool.requests.insert(id, notifier);
		// 						},
		// 						Call::Batch(RawCall { id, request, notifier }) => {
		// 							tracing::trace!("BatchRequests({request})");

		// 							ws_tx.send(Message::Text(request)).await.unwrap();
		// 							pool.batches.insert(id, notifier);
		// 						},
		// 					}
		// 				} else {
		// 					//
		// 				}

		// 				rxs_fut = future::select(rx.next(), maybe_response_fut);
		// 				interval_fut = interval_fut_;
		// 			},
		// 			Left((Right((maybe_response, maybe_call_fut)), interval_fut_)) => {
		// 				if let Some(response) = maybe_response {
		// 					pool.on_ws_recv(response).await.unwrap()
		// 				} else {
		// 					//
		// 				}

		// 				rxs_fut = future::select(maybe_call_fut, ws_rx.next());
		// 				interval_fut = interval_fut_;
		// 			},
		// 			Right((_, rxs_fut_)) => {
		// 				tracing::trace!("TickRequest({system_health_request})");

		// 				ws_tx.send(Message::Text(system_health_request.clone())).await.unwrap();

		// 				rxs_fut = rxs_fut_;
		// 				interval_fut = interval.next().fuse();
		// 			},
		// 		}
		// 	}
		// });

		Ok(Ws {
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
impl Default for Initializer {
	fn default() -> Self {
		Self {
			concurrency_limit: 512,
			interval: Duration::from_secs(10),
			request_timeout: Duration::from_secs(10),
		}
	}
}

/// Ws instance.
///
/// Use this to interact with the server.
pub struct Ws {
	messenger: Messenger,
	request_queue: RequestQueue,
	request_timeout: Duration,
}
impl Ws {
	const VERSION: &'static str = "2.0";

	/// Send a single request.
	pub async fn request<'a, D, R>(&self, raw_request: R) -> Result<Response<D>>
	where
		D: DeserializeOwned,
		R: Into<RawRequest<'a, Value>>,
	{
		let RequestQueueGuard { lock: id, .. } = self.request_queue.next()?;
		let RawRequest { method, params } = raw_request.into();
		let (tx, rx) = oneshot::channel();

		// Debug.
		// self.messenger.send(Call::Debug(id)).await.map_err(|_| error::Tokio::MpscSend)?;
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
			return Ok(Vec::new());
			// Err(error::Jsonrpc::EmptyBatch)?;
		}

		let RequestQueueGuard { lock: ids, .. } = self.request_queue.take(raw_requests.len())?;
		let id = ids.first().expect("[core::jsonrpc] `raw_requests` never empty; qed").to_owned();
		let requests = ids
			.into_iter()
			.zip(raw_requests.into_iter())
			.map(|(id, raw_request)| {
				let RawRequest { method, params } = raw_request.into();

				Request { jsonrpc: Self::VERSION, id, method, params }
			})
			.collect::<Vec<_>>();
		let request = serde_json::to_string(&requests).map_err(error::Generic::Serde)?;
		let (tx, rx) = oneshot::channel();

		self.messenger
			.send(Call::Batch(RawCall { id, request, notifier: tx }))
			.await
			.map_err(|_| error::Tokio::MpscSend)?;

		let mut responses = time::timeout(self.request_timeout, rx)
			.await
			.map_err(error::Tokio::Elapsed)?
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
					Message::Binary(raw_response) =>
						self.process_raw_response(str::from_utf8(&raw_response).unwrap()).await,
					Message::Text(raw_response) => self.process_raw_response(&raw_response).await,
					Message::Ping(_) => tracing::warn!("ping"),
					Message::Pong(_) => tracing::warn!("pong"),
					Message::Close(_) => tracing::warn!("close"),
					Message::Frame(_) => tracing::warn!("frame"),
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
				tracing::trace!("TickResponse({raw_response})");

				return;
			}

			tracing::trace!("RequestResponse({raw_response})");

			let notifier = self.requests.remove(&response.id).unwrap();

			if let Err(e) = notifier.send(response) {
				tracing::error!("{e:?}");
			}
		} else if let Ok(responses) = serde_json::from_str::<BatchResponse>(raw_response) {
			tracing::trace!("BatchResponse({raw_response})");

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
	// Debug.
	// Debug(Id),
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
