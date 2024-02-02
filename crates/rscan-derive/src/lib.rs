use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Expr, ItemFn, LitStr, Token};

struct ModuleArgs {
    name: LitStr,
    description: LitStr,
    author: LitStr,
    version: LitStr,
    category: LitStr,
    platforms: LitStr,
}

impl Parse for ModuleArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![,]>()?;
        let description = input.parse()?;
        input.parse::<Token![,]>()?;
        let author = input.parse()?;
        input.parse::<Token![,]>()?;
        let version = input.parse()?;
        input.parse::<Token![,]>()?;
        let category = input.parse()?;
        input.parse::<Token![,]>()?;
        let platforms = input.parse()?;

        Ok(ModuleArgs {
            name,
            description,
            author,
            version,
            category,
            platforms,
        })
    }
}

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ModuleArgs);
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let identifier = fn_name.to_string().to_lowercase(); // Derive identifier
    let ModuleArgs {
        name,
        description,
        author,
        version,
        category,
        platforms,
    } = args;

    let kind_expr = quote! { rscan_core::ModuleKind::Sync(#fn_name) };

    let expanded = quote! {
        #input_fn

        inventory::submit! {
            rscan_core::Module {
                name: #name,
                identifier: &#identifier,
                description: #description,
                author: #author,
                version: #version,
                category: #category,
                kind: #kind_expr,
                platforms: rscan_core::parse_platforms(#platforms),

            }
        }
    };

    TokenStream::from(expanded)
}
