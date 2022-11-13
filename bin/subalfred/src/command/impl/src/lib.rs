// proc-macro
use proc_macro::TokenStream;
// crates.io
use syn::*;

/// Quickly define and implement a command containing serval subcommands.
#[proc_macro_attribute]
pub fn cmd(_: TokenStream, input: TokenStream) -> TokenStream {
	let cmd_enum = syn::parse_macro_input!(input as ItemEnum);

	// #[cfg(feature = "debug")]
	// dbg!(&cmd_enum);

	let ItemEnum {
		attrs: cmd_attrs, vis: cmd_vis, ident: cmd_name, variants: cmd_variants, ..
	} = cmd_enum;
	let cmd_variants_names =
		cmd_variants.iter().map(|variant| variant.ident.clone()).collect::<Vec<_>>();
	let cmd_variants = cmd_variants
		.into_iter()
		.map(|Variant { attrs, ident, .. }| {
			let cmd = quote::format_ident!("{ident}Cmd");

			quote::quote! {
				#(#attrs)*
				#ident(#cmd)
			}
		})
		.collect::<Vec<_>>();

	quote::quote! {
		#[derive(Debug, clap::Subcommand)]
		#(#cmd_attrs)*
		#cmd_vis enum #cmd_name {
			#(#cmd_variants,)*
		}
		impl #cmd_name {
			#cmd_vis fn run(&self) -> crate::prelude::Result<()> {
				match self {
					#(
						Self::#cmd_variants_names(cmd) => cmd.run(),
					)*
				}
			}
		}
	}
	.into()
}
