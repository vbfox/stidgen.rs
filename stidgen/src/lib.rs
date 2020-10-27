#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::ptr_arg)]

extern crate proc_macro;

mod impls;
mod options;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, parse_macro_input, Ident};

macro_rules! add_impl_if_enabled {
    ( $option:expr, $impl:expr ) => {{
        if $option {
            $impl
        } else {
            proc_macro2::TokenStream::new()
        }
    }};
}

struct Stidgen<'a> {
    resolved_options: &'a options::Resolved,
    name: &'a Ident,
    item_ast: &'a syn::ItemStruct,
}

impl<'a> Stidgen<'a> {
    pub fn new(item_ast: &'a syn::ItemStruct, resolved_options: &'a options::Resolved) -> Self {
        Self {
            resolved_options,
            name: &item_ast.ident,
            item_ast,
        }
    }

    pub fn to_tokens(&self) -> TokenStream2 {
        let name = self.name;
        let options = self.resolved_options;
        let item_ast = self.item_ast;

        let clone = add_impl_if_enabled!(options.clone, impls::clone(name));
        let hash = add_impl_if_enabled!(options.hash, impls::hash(name));
        let eq = add_impl_if_enabled!(options.eq, impls::eq(name));
        let partial_eq = add_impl_if_enabled!(options.partial_eq, impls::partial_eq(name));
        let ord = add_impl_if_enabled!(options.ord, impls::ord(name));
        let partial_ord = add_impl_if_enabled!(options.partial_ord, impls::partial_ord(name));
        let display = add_impl_if_enabled!(options.display, impls::display(name));
        let debug = add_impl_if_enabled!(options.debug, impls::debug(name));
        let as_bytes = add_impl_if_enabled!(options.as_bytes, impls::as_bytes(name));
        let into_inner = add_impl_if_enabled!(options.into_inner, impls::into_inner(name));
        let new = add_impl_if_enabled!(options.new, impls::new(name));
        let as_ref = add_impl_if_enabled!(options.as_ref, impls::as_ref(name));
        let borrow = add_impl_if_enabled!(options.borrow, impls::borrow(name));
        let as_str = add_impl_if_enabled!(options.as_str, impls::as_str(name));

        quote! {
            #item_ast
            #new
            #into_inner
            #clone
            #hash
            #eq
            #partial_eq
            #ord
            #partial_ord
            #display
            #debug
            #as_ref
            #borrow
            #as_bytes
            #as_str
        }
    }
}

fn impl_string_id(_attr_ast: &syn::AttributeArgs, item_ast: &syn::ItemStruct) -> TokenStream {
    let known_type = Some(options::KnownTypes::String);
    let user_options = options::Options::default();
    let options = user_options.resolve_for(known_type);

    let gen = Stidgen::new(item_ast, &options);

    TokenStream::from(gen.to_tokens())
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
