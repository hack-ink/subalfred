// proc-macro
use proc_macro::TokenStream;
// crates.io
use proc_macro2::TokenStream as TokenStream2;
use syn::*;

/// Quickly define and implement a command containing serval subcommands.
#[proc_macro_attribute]
pub fn cmd(_: TokenStream, input: TokenStream) -> TokenStream {
	let item_enum = syn::parse_macro_input!(input as ItemEnum);

	// #[cfg(feature = "debug")]
	// dbg!(&item_enum);

	let ItemEnum { attrs, vis, ident, variants, .. } = item_enum;
	let variant_runs = variants
		.iter()
		.map(|v| v.ident.to_string().parse::<TokenStream2>().unwrap())
		.map(|n| {
			quote::quote! {
				Self::#n(cmd) => { cmd.run() }
			}
		})
		.collect::<Vec<_>>();
	let variants = variants
		.into_iter()
		.map(|Variant { attrs, ident, .. }| {
			let cmd = format!("{}Cmd", ident).parse::<TokenStream2>().unwrap();

			quote::quote! {
				#(#attrs)*
				#ident(#cmd)
			}
		})
		.collect::<Vec<_>>();

	quote::quote! {
		#[derive(Debug, clap::Subcommand)]
		#(#attrs)*
		#vis enum #ident {
			#(#variants,)*
		}
		impl #ident {
			#vis fn run(&self) -> crate::prelude::Result<()> {
				match self {
					#(#variant_runs,)*
				}
			}
		}
	}
	.into()
}
