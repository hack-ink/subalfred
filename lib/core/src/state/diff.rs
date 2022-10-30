//! Diff core library.

// std
use std::path::Path;
// hack-ink
use crate::prelude::*;

/// Check the differences between the two states.
/// Note:
/// This is not a symmetric difference operation.
/// `a.diff(b)` might equal `b.diff(a)`, but not always.
pub fn diff<P>(a: P, b: P) -> Result<Vec<String>>
where
	P: Send + AsRef<Path>,
{
	subalfred_util::execution_timer!("state::diff");

	if a.as_ref() == b.as_ref() {
		return Ok(Vec::new());
	}

	let (a, b) = super::read_chain_spec_concurrent(a, b)?;
	let (a, mut b) = (a.genesis.raw.top, b.genesis.raw.top);
	let mut diff = Vec::new();

	for (a_k, a_v) in a {
		if let Some(b_v) = b.remove(&a_k) {
			// Different value under the same key.
			if a_v != b_v {
				diff.push(format!("-{a_k}:{a_v}\n+{a_k}:{b_v}"));
			}

		// Completely the same.
		}
		// Keys only appear in a.
		else {
			diff.push(format!("-{a_k}:{a_v}"));
		}
	}
	// Keys only appear in b.
	for (k, v) in b {
		diff.push(format!("+{k}:{v}"));
	}

	Ok(diff)
}
