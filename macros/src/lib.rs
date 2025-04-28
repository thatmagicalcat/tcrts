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

struct ListInput {
    items: Vec<syn::Type>,
}

impl syn::parse::Parse for ListInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = vec![];
        while let Ok(t) = input.parse::<syn::Type>() {
            items.push(t);
            if input.parse::<syn::Token![,]>().is_err() {
                break;
            }
        }

        Ok(Self { items })
    }
}

#[proc_macro]
pub fn list(input: TokenStream) -> TokenStream {
    let ListInput { mut items } = syn::parse_macro_input!(input as ListInput);

    items.reverse();
    let mut expr = quote! { Nil };

    for item in items {
        expr = quote! { Cons<#item, #expr>  };
    }

    expr.into()
}

struct ListToArrayInput {
    list: syn::Type,
    _comma: syn::Token![,],
    start: syn::LitInt,
    _dots: syn::Token![..],
    eq: Option<syn::Token![=]>,
    end: syn::LitInt,
}

impl syn::parse::Parse for ListToArrayInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            list: input.parse()?,
            _comma: input.parse()?,
            start: input.parse()?,
            _dots: input.parse()?,
            eq: input.parse()?,
            end: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn list_to_array(input: TokenStream) -> TokenStream {
    let ListToArrayInput {
        list,
        start,
        end,
        eq,
        ..
    } = syn::parse_macro_input!(input as ListToArrayInput);

    let start = start.base10_parse::<usize>().unwrap();
    let end = end.base10_parse::<usize>().unwrap();

    assert!(start < end);

    let range = if eq.is_some() {
        start..end + 1
    } else {
        start..end
    };

    // types used for indexing
    let indexing_type = range.map(|i| {
        repeat_internal(
            i,
            quote! { ts_abuse::number::Next },
            quote! { ts_abuse::number::Zero },
        )
    });

    quote! {[
        #(<#list as ts_abuse::list::index::GetIndex<#indexing_type>>::Output::VALUE),*
    ]}.into()
}
