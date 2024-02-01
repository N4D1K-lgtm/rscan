extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let module_name = if args.is_empty() {
        fn_name.to_string()
    } else {
        parse_macro_input!(args as syn::LitStr).value()
    };

    // Generate the code to register the module
    let expanded = quote! {
        #input_fn

        inventory::submit! {
            rscan_core::Module {
                name: #module_name,
                kind: rscan_core::ModuleKind::Sync(#fn_name),
            }
        }
    };

    TokenStream::from(expanded)
}
