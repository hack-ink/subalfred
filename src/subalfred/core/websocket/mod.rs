// std
use std::collections::HashMap;
// crates.io
use futures::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::sync::{
	mpsc::{self, Receiver as MpscReceiver, Sender as MpscSender},
	oneshot::{self, Sender as OneshotSender},
};
use tokio_tungstenite::tungstenite::Message as WsMessage;
// hack-ink
use crate::core::{jsonrpc::Response, Result};

/// TODO
pub struct Client {
	tx: MpscSender<Message>,
	// pool: HashMap<usize>,
}
impl Client {
	/// TODO
	pub async fn connect() -> Self {
		let (mut ws_tx, mut ws_rx) =
			tokio_tungstenite::connect_async("wss://pangoro-rpc.darwinia.network")
				.await
				.unwrap()
				.0
				.split();
		let (tx, mut rx) = mpsc::channel(512);

		tokio::spawn(async move {
			loop {
				tokio::select! {
					msg = ws_rx.next() => {
						if let Some(msg) = msg {
							if let Ok(WsMessage::Text(text)) = msg {
							}
						}
					}
					msg = rx.recv() => {
						if let Some(msg) = msg {
							match msg {
								Message::Request(req) => {
									ws_tx.send(WsMessage::Text(req.content)).await;
								}
								_ => todo!()
							}
						}
					}
				}
			}
		});

		Self { tx }
	}

	/// TODO
	pub async fn send_jsonrpc<T>(_json: &Value) -> Result<Response<T>>
	where
		T: DeserializeOwned,
	{
		todo!()
	}
}

/// TODO
#[derive(Debug)]
enum Message {
	Request(Request),
	Requests(()),
}

/// TODO
#[derive(Debug)]
struct Request {
	id: usize,
	content: String,
	notifier: OneshotSender<MpscReceiver<Response<String>>>,
}
