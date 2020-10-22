#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::ptr_arg)]

extern crate proc_macro;

mod impls;
mod options;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input};

macro_rules! add_impl_if_enabled {
    ( $option:expr, $impl:expr ) => {{
        if $option {
            $impl
        } else {
            proc_macro2::TokenStream::new()
        }
    }};
}

fn impl_string_id(_attr_ast: &syn::AttributeArgs, item_ast: &syn::ItemStruct) -> TokenStream {
    let name = &item_ast.ident;
    let known_type = Some(options::KnownTypes::String);
    let user_options = options::Options::default();
    let options = user_options.resolve_for(known_type);

    let clone = add_impl_if_enabled!(options.clone, impls::clone(name));
    let hash = add_impl_if_enabled!(options.hash, impls::hash(name));
    let eq = add_impl_if_enabled!(options.eq, impls::eq(name));
    let partial_eq = add_impl_if_enabled!(options.partial_eq, impls::partial_eq(name));
    let ord = add_impl_if_enabled!(options.ord, impls::ord(name));
    let partial_ord = add_impl_if_enabled!(options.partial_ord, impls::partial_ord(name));
    let display = add_impl_if_enabled!(options.display, impls::display(name));
    let debug = add_impl_if_enabled!(options.debug, impls::debug(name));
    let as_bytes = add_impl_if_enabled!(options.as_bytes, impls::as_bytes(name));

    let gen = quote! {
        #item_ast

        #[automatically_derived]
        impl #name {
            #[inline]
            pub fn new<S: Into<String>>(s: S) -> #name {
                #name(s.into())
            }

            #[inline]
            pub fn into_string(self) -> String {
                self.0
            }
        }

        #clone
        #hash
        #eq
        #partial_eq
        #ord
        #partial_ord

        #[automatically_derived]
        impl ::std::convert::Into<String> for #name {
            #[inline]
            fn into(self) -> String {
                self.0
            }
        }

        #display
        #debug

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

        #as_bytes
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
