//! The core library about how Subalfred interacts with JSONRPC.

pub mod http;
pub mod ws;

// std
use std::sync::{
	atomic::{AtomicUsize, Ordering},
	Arc,
};
// crates.io
use serde::{Deserialize, Serialize};
// hack-ink
use crate::core::{error, Result};

/// JSONRPC Id.
pub type Id = usize;

const E_REQUEST_QUEUE_GUARD_BROKE: &str = "[core::jsonrpc] request queue guard broke";

/// Generic JSONRPC request.
#[allow(missing_docs)]
#[derive(Debug, Serialize)]
pub struct Request<'a, P> {
	#[serde(borrow)]
	pub jsonrpc: &'a str,
	pub id: Id,
	#[serde(borrow)]
	pub method: &'a str,
	pub params: P,
}
/// Raw JSONRPC request.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct RawRequest<'a, P> {
	pub method: &'a str,
	pub params: P,
}
impl<'a, P> From<(&'a str, P)> for RawRequest<'a, P> {
	fn from(raw: (&'a str, P)) -> Self {
		Self { method: raw.0, params: raw.1 }
	}
}

/// Generic JSONRPC response.
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct Response<R> {
	pub jsonrpc: String,
	pub id: Id,
	pub result: R,
}

#[derive(Debug)]
struct RequestQueue {
	size: Id,
	active: Arc<()>,
	next: AtomicUsize,
}
impl RequestQueue {
	fn next(&self) -> Result<RequestQueueGuard<Id>> {
		let active = Arc::strong_count(&self.active);

		tracing::trace!("RequestQueue({active}/{})", self.size);

		if active == self.size {
			Err(error::Jsonrpc::ExceededRequestQueueMaxSize(self.size))?
		} else {
			Ok(RequestQueueGuard {
				lock: self.next.fetch_add(1, Ordering::SeqCst),
				_strong: self.active.clone(),
			})
		}
	}

	fn take(&self, count: Id) -> Result<RequestQueueGuard<Vec<Id>>> {
		let active = Arc::strong_count(&self.active);

		tracing::trace!("RequestQueue({active}/{})", self.size);

		if active == self.size {
			Err(error::Jsonrpc::ExceededRequestQueueMaxSize(self.size))?
		} else {
			Ok(RequestQueueGuard {
				lock: (0..count).map(|_| self.next.fetch_add(1, Ordering::SeqCst)).collect(),
				_strong: self.active.clone(),
			})
		}
	}
}

struct RequestQueueGuard<L> {
	lock: L,
	_strong: Arc<()>,
}
