// proc-macro
use proc_macro::TokenStream;
// crates.io
use proc_macro2::TokenStream as TokenStream2;
use syn::*;

/// `once_call`'s name must end with `_once`.
#[proc_macro_attribute]
pub fn rpc(_: TokenStream, input: TokenStream) -> TokenStream {
	let once_call = syn::parse_macro_input!(input as ItemFn);

	// #[cfg(feature = "debug")]
	// dbg!(&once_call);

	// Names
	// get_block_hash_once
	// ->
	// get_block_hash
	let call_name = once_call
		.sig
		.ident
		.to_string()
		.rsplit_once('_')
		.unwrap()
		.0
		.parse::<TokenStream2>()
		.unwrap();
	// get_block_hash_once
	// ->
	// get_block_hash_raw
	let raw_call_name = format!("{}_raw", call_name).parse::<TokenStream2>().unwrap();
	// Inputs
	let raw_call_inputs = once_call.sig.inputs.clone();
	// block_number: impl Serialize
	// ->
	// id: usize, block_number: impl Serialize
	let call_inputs = {
		let inputs = once_call.sig.inputs.clone();

		quote::quote! {
			id: usize, #inputs
		}
	};
	// Outputs
	// -> Value
	let output = once_call.sig.output.clone();
	// Call blocks
	// {
	// 	crate::rpc_once("chain_getBlockHash", serde_json::json!([block_number]))
	// }
	// ->
	// {
	// 	("chain_getBlockHash", serde_json::json!([block_number]))
	// }
	// {
	// 	crate::rpc(id, "chain_getBlockHash", serde_json::json!([block_number]))
	// }
	let (raw_call_block, call_block) = {
		let args = if let Stmt::Expr(Expr::Call(c)) = &once_call.block.stmts[0] {
			c.args.clone()
		} else {
			unreachable!()
		};

		(quote::quote! {{ (#args) }}, quote::quote! {{ crate::rpc(id, #args) }})
	};

	quote::quote! {
		#once_call
		pub fn #raw_call_name(#raw_call_inputs) -> (&'static str, Value) #raw_call_block
		pub fn #call_name(#call_inputs) #output #call_block
	}
	.into()
}
