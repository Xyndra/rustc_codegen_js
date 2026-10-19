use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, LitStr};

fn path_from_attr(attr: TokenStream) -> LitStr {
    if attr.is_empty() {
        return syn::parse_str("\"/\"").unwrap();
    }
    syn::parse(attr).unwrap_or_else(|_| syn::parse_str("\"/\"").unwrap())
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    quote!(#item).into()
}

#[proc_macro_attribute]
pub fn page(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    let path = path_from_attr(attr);
    quote! {
        #item
        ::fw_router::inventory::submit! {
            ::fw_router::RegisteredRoute {
                path: #path,
                method: ::fw_router::Method::Get,
                cache: ::fw_router::CachePolicy::Cache,
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn rpc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    let path = path_from_attr(attr);
    quote! {
        #item
        ::fw_router::inventory::submit! {
            ::fw_router::RegisteredRoute {
                path: #path,
                method: ::fw_router::Method::Get,
                cache: ::fw_router::CachePolicy::None,
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn rest(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    let path = path_from_attr(attr);
    quote! {
        #item
        ::fw_router::inventory::submit! {
            ::fw_router::RegisteredRoute {
                path: #path,
                method: ::fw_router::Method::Post,
                cache: ::fw_router::CachePolicy::None,
            }
        }
    }
    .into()
}

/// Marks an item as server-only (`#[cfg(feature = "server")]`).
#[proc_macro_attribute]
pub fn server(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    quote! {
        #[cfg(feature = "server")]
        #item
    }
    .into()
}

/// Marks an item as client-only (`#[cfg(feature = "client")]`).
#[proc_macro_attribute]
pub fn client(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    quote! {
        #[cfg(feature = "client")]
        #item
    }
    .into()
}

/// Marks a page module as gated behind its `page_<name>` feature.
#[proc_macro_attribute]
pub fn page_feature(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = if attr.is_empty() {
        String::new()
    } else {
        attr.to_string().trim().to_string()
    };
    let feature = format!("page_{name}");
    let item = parse_macro_input!(item as Item);
    quote! {
        #[cfg(feature = #feature)]
        #item
    }
    .into()
}

/// Entry point for a server binary: keeps the annotated `main` for server
/// builds and substitutes an empty `main` otherwise, so no `cfg` is needed.
#[proc_macro_attribute]
pub fn server_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    quote! {
        #[cfg(feature = "server")]
        #item

        #[cfg(not(feature = "server"))]
        fn main() {}
    }
    .into()
}

#[proc_macro]
pub fn router(_input: TokenStream) -> TokenStream {
    quote!(::fw_router::build_router()).into()
}



#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    use proc_macro2::{Delimiter, TokenStream as TS2, TokenTree};

    #[derive(Clone, Copy, PartialEq)]
    enum Kind {
        Word,
        Punct,
        Placeholder,
        Close,
    }

    fn walk(ts: TS2, out: &mut String, args: &mut Vec<TS2>, prev: &mut Option<Kind>) {
        for tt in ts {
            match tt {
                TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                    if matches!(prev, Some(Kind::Word) | Some(Kind::Placeholder)) {
                        out.push(' ');
                    }
                    out.push_str("{}");
                    args.push(g.stream());
                    *prev = Some(Kind::Placeholder);
                }
                TokenTree::Ident(id) => {
                    if matches!(prev, Some(Kind::Word) | Some(Kind::Placeholder)) {
                        out.push(' ');
                    }
                    out.push_str(&id.to_string());
                    *prev = Some(Kind::Word);
                }
                TokenTree::Literal(lit) => {
                    if matches!(prev, Some(Kind::Word) | Some(Kind::Placeholder)) {
                        out.push(' ');
                    }
                    out.push_str(&lit.to_string().replace('{', "{{").replace('}', "}}"));
                    *prev = Some(Kind::Word);
                }
                TokenTree::Punct(p) => {
                    let ch = p.as_char();
                    out.push(ch);
                    *prev = if ch == '>' || ch == ')' || ch == ']' {
                        Some(Kind::Close)
                    } else {
                        Some(Kind::Punct)
                    };
                }
                TokenTree::Group(g) => {
                    let (open, close) = match g.delimiter() {
                        Delimiter::Parenthesis => ("(", ")"),
                        Delimiter::Bracket => ("[", "]"),
                        _ => ("", ""),
                    };
                    out.push_str(open);
                    walk(g.stream(), out, args, prev);
                    out.push_str(close);
                    *prev = Some(Kind::Close);
                }
            }
        }
    }

    let mut fmt = String::new();
    let mut args = Vec::new();
    let mut prev = None;
    walk(input.into(), &mut fmt, &mut args, &mut prev);
    let fmt_lit = proc_macro2::Literal::string(&fmt);
    if args.is_empty() {
        quote! { format!(#fmt_lit) }.into()
    } else {
        quote! { format!(#fmt_lit, #(#args),*) }.into()
    }
}
