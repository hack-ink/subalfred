// proc-macro
use proc_macro::TokenStream;
// crates.io
use proc_macro2::TokenStream as TokenStream2;
use syn::*;

/// The call name must end with `_once`.
#[proc_macro_attribute]
pub fn rpc(_: TokenStream, input: TokenStream) -> TokenStream {
	let call = syn::parse_macro_input!(input as ItemFn);

	#[cfg(feature = "debug")]
	dbg!(&call);

	// get_block_hash_once
	// ->
	// get_block_hash
	let call_name =
		call.sig.ident.to_string().rsplit_once("_").unwrap().0.parse::<TokenStream2>().unwrap();
	// block_number: impl Serialize
	// ->
	// id: u32, block_number: impl Serialize
	let call_inputs = {
		let inputs = call.sig.inputs.clone();

		quote::quote! {
			id: u32, #inputs
		}
	};
	// -> Value
	let output = call.sig.output.clone();
	// {
	// 	crate::rpc_once("chain_getBlockHash", json!([block_number]))
	// }
	// ->
	// {
	// 	crate::rpc(id, "chain_getBlockHash", json!([block_number]))
	// }
	let call_block = {
		let args = if let Stmt::Expr(Expr::Call(c)) = &call.block.stmts[0] {
			c.args.clone()
		} else {
			unreachable!()
		};

		quote::quote! {{
			crate::rpc(id, #args)
		}}
	};
	let token_stream = quote::quote! {
		#call
		pub fn #call_name(#call_inputs) #output #call_block
	};

	token_stream.into()
}
