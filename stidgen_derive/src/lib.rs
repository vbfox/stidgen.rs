extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input};

fn impl_string_id(_attr_ast: &syn::AttributeArgs, item_ast: &syn::ItemStruct) -> TokenStream {
    let name = &item_ast.ident;
    let gen = quote! {
        #[derive(std::fmt::Debug, Clone, PartialEq, Eq, Hash)]
        #item_ast

        impl #name {
            pub fn new<S: Into<String>>(s: S) -> #name {
                #name(s.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn to_string(&self) -> String {
                self.0.clone()
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl std::convert::Into<String> for #name {
            fn into(self) -> String {
                self.0
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };

    TokenStream::from(gen)
}

#[proc_macro_attribute]
pub fn string_id(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let attr_ast = parse_macro_input!(attr as syn::AttributeArgs);
    let item_ast = parse_macro_input!(item as syn::ItemStruct);

    // Build the trait implementation
    impl_string_id(&attr_ast, &item_ast)
}
