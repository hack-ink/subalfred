//! Collections of Subalfred utilities.

#![warn(missing_docs)]

// std
use std::time::Instant;

/// Initialize an [`ExecutionTimer`] quickly.
#[macro_export]
macro_rules! execution_timer {
	($meta:expr) => {
		let _execution_timer = $crate::ExecutionTimer::init(|start| {
			tracing::trace!("{} takes {} secs", $meta, start.elapsed().as_secs_f64())
		});
	};
}

/// A timer to measure the elapsed time of code block.
///
/// This struct holding a start time.
/// While it get dropped, it will log the elapsed time.
/// # Examples
/// ```
/// use subalfred_util::ExecutionTimer;
///
/// // Must give a name to the timer, otherwise it will be dropped immediately.
/// let _t = ExecutionTimer::init(|start| {
/// 	tracing::trace!("`example` takes `{}` secs", start.elapsed().as_secs_f64())
/// });
/// ```
#[allow(clippy::tabs_in_doc_comments)]
pub struct ExecutionTimer<T>
where
	T: FnOnce(Instant),
{
	/// The start time.
	pub start: Instant,
	_tracing: Option<T>,
}
impl<T> ExecutionTimer<T>
where
	T: FnOnce(Instant),
{
	/// Initialize the timer.
	pub fn init(tracing: T) -> Self {
		Self { start: Instant::now(), _tracing: Some(tracing) }
	}
}
impl<T> Drop for ExecutionTimer<T>
where
	T: FnOnce(Instant),
{
	fn drop(&mut self) {
		let _ = self._tracing.take().map(|f| f(self.start));
	}
}

#[test]
fn execution_timer_should_work() {
	tracing_subscriber::fmt::init();
	execution_timer!("execution_timer_should_work");
}
