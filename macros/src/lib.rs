use proc_macro::TokenStream;
use quote::quote;

struct ApplyInput {
    wrapper: syn::Ident,
    _comma1: syn::Token![,],
    inner: syn::Expr,
    _comma2: syn::Token![,],
    count: syn::LitInt,
}

impl syn::parse::Parse for ApplyInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ApplyInput {
            wrapper: input.parse()?,
            _comma1: input.parse()?,
            inner: input.parse()?,
            _comma2: input.parse()?,
            count: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn apply(input: TokenStream) -> TokenStream {
    let ApplyInput {
        wrapper,
        inner,
        count,
        ..
    } = syn::parse_macro_input!(input as ApplyInput);

    let n = count.base10_parse::<usize>().unwrap();
    repeat_internal(n, quote! { #wrapper }, quote! { #inner }).into()
}

fn repeat_internal(
    n: usize,
    wrapper: proc_macro2::TokenStream,
    inner: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let mut expr = quote! { #inner };

    for _ in 0..n {
        expr = quote! { #wrapper < #expr > };
    }

    expr
}
