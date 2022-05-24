// proc-macro
use proc_macro::TokenStream;
// crates.io
use proc_macro2::TokenStream as TokenStream2;
use syn::{
	punctuated::Punctuated, spanned::Spanned, token::Comma, Expr, ExprPath, FnArg, Ident, ItemFn,
	Pat, Path, PathArguments, PathSegment, Stmt,
};

#[proc_macro_attribute]
pub fn rpc(_: TokenStream, input: TokenStream) -> TokenStream {
	let call = syn::parse_macro_input!(input as ItemFn);

	// dbg!(&call);

	let call_vis = call.vis;
	let call_sig = call.sig;
	let call_name = call_sig.ident.clone();
	let call_with_id_name =
		format!("{}_with_id", call_name.to_string()).parse::<TokenStream2>().unwrap();
	let inputs = call_sig.inputs.clone();
	let input_names = inputs
		.iter()
		.map(|a| {
			if let FnArg::Typed(t) = a {
				if let Pat::Ident(i) = &*t.pat {
					i.ident.clone()
				} else {
					unimplemented!()
				}
			} else {
				unimplemented!()
			}
		})
		.collect::<Punctuated<_, Comma>>();
	let output = call_sig.output.clone();
	let (call_with_id_block, default_id) = {
		let mut call_with_id_block = call.block;
		let mut default_id = "DEFAULT_ID".parse::<TokenStream2>().unwrap();

		if let Stmt::Expr(Expr::Call(c)) = &mut call_with_id_block.stmts[0] {
			if let Expr::Path(p) = &mut c.args[0] {
				let i = &mut p.path.segments[0].ident;

				default_id = i.to_string().parse().unwrap();
				*i = Ident::new("id", i.span());
			}
		}

		(call_with_id_block, default_id)
	};
	let call_with_raw_params_and_id_block = {
		let mut call_with_raw_params_and_id_block = call_with_id_block.clone();

		if let Stmt::Expr(Expr::Call(c)) = &mut call_with_raw_params_and_id_block.stmts[0] {
			if let Some(m) = c.args.pop() {
				c.args.push(Expr::Path(ExprPath {
					attrs: Vec::new(),
					qself: None,
					path: Path {
						leading_colon: None,
						segments: vec![PathSegment {
							ident: Ident::new("params", m.span()),
							arguments: PathArguments::None,
						}]
						.into_iter()
						.collect(),
					},
				}));
			}
		}

		call_with_raw_params_and_id_block
	};
	let mut token_stream = quote::quote! {
		#call_vis #call_sig {
			#call_with_id_name(#default_id, #input_names)
		}
		#call_vis fn #call_with_id_name(id: impl Serialize, #inputs) #output #call_with_id_block
	};

	if !inputs.is_empty() {
		let call_with_raw_params_name =
			format!("{}_with_raw_params", call_name.to_string()).parse::<TokenStream2>().unwrap();
		let call_with_raw_params_and_id_name =
			format!("{}_and_id", call_with_raw_params_name.to_string())
				.parse::<TokenStream2>()
				.unwrap();

		token_stream.extend(quote::quote! {
			#[cfg(feature = "raw-params")]
			#call_vis fn #call_with_raw_params_and_id_name(id: impl Serialize, params: impl Serialize) #output
			#call_with_raw_params_and_id_block
			#[cfg(feature = "raw-params")]
			#call_vis fn #call_with_raw_params_name(params: impl Serialize) #output {
				#call_with_raw_params_and_id_name(#default_id, params)
			}
		});
	}

	token_stream.into()
}
