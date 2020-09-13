#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::ptr_arg)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input};

fn impl_string_id(_attr_ast: &syn::AttributeArgs, item_ast: &syn::ItemStruct) -> TokenStream {
    let name = &item_ast.ident;
    let gen = quote! {
        #item_ast

        #[automatically_derived]
        impl #name {
            #[inline]
            pub fn new<S: Into<String>>(s: S) -> #name {
                #name(s.into())
            }

            #[inline]
            pub fn to_string(&self) -> String {
                self.0.clone()
            }

            #[inline]
            pub fn into_string(self) -> String {
                self.0
            }
        }

        #[automatically_derived]
        impl ::std::clone::Clone for #name {
            #[inline]
            fn clone(&self) -> Self {
                #name(self.0.clone())
            }
        }

        #[automatically_derived]
        impl ::std::hash::Hash for #name {
            #[inline]
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        #[automatically_derived]
        impl ::std::cmp::PartialEq for #name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        #[automatically_derived]
        impl ::std::cmp::Eq for #name {}

        #[automatically_derived]
        impl ::std::cmp::PartialOrd for #name {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        #[automatically_derived]
        impl ::std::cmp::Ord for #name {
            #[inline]
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        #[automatically_derived]
        impl ::std::convert::Into<String> for #name {
            #[inline]
            fn into(self) -> String {
                self.0
            }
        }

        #[automatically_derived]
        impl ::std::fmt::Display for #name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        #[automatically_derived]
        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!(#name))
                 .field(&self.0)
                 .finish()
            }
        }

        #[automatically_derived]
        impl ::std::borrow::Borrow<str> for #name {
            #[inline]
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        #[automatically_derived]
        impl ::std::convert::AsRef<str> for #name {
            #[inline]
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        #[automatically_derived]
        impl #name {
            /// Extracts a string slice containing the entire ID.
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        #[automatically_derived]
        impl ::std::convert::AsRef<[u8]> for #name {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                ::std::convert::AsRef::<[u8]>::as_ref(&self.0)
            }
        }

        #[automatically_derived]
        impl #name {
            /// Returns a byte slice of this ID's contents.
            #[inline]
            pub fn as_bytes(&self) -> &[u8] {
                ::std::convert::AsRef::<[u8]>::as_ref(&self.0)
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
