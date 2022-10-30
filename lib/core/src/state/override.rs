//! Override core library.

// std
use std::path::Path;
// hack-ink
use crate::prelude::*;

/// Override the state a with b.
pub fn r#override<P>(a: P, b: P) -> Result<()>
where
	P: Send + AsRef<Path>,
{
	subalfred_util::execution_timer!("override state");

	let (a, b) = (a.as_ref(), b.as_ref());

	if a == b {
		return Ok(());
	}

	let (a_spec, b_spec) = super::read_chain_spec_concurrent(a, b)?;
	let a_spec = super::override_top(a_spec, b_spec);

	super::write_to_custom_extension_file(a, "override", a_spec)
}
