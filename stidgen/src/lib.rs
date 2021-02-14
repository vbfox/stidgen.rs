#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::ptr_arg)]

extern crate proc_macro;

mod impls;
mod known_types;
mod options;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, parse_macro_input, Ident, Type};

macro_rules! add_impl_if_enabled {
    ( $option:expr, $impl:expr ) => {{
        if $option {
            $impl
        } else {
            proc_macro2::TokenStream::new()
        }
    }};
}

#[derive(Debug)]
enum TypeInfoError {
    NamedFields,
    UnitFields,
    InvalidFieldCount { count: usize },
}

struct IdTypeInfo {
    name: Ident,
    inner_type: Type,
}

impl IdTypeInfo {
    pub fn new(item_ast: &syn::ItemStruct) -> Result<Self, TypeInfoError> {
        match item_ast.fields {
            syn::Fields::Named(_) => Err(TypeInfoError::NamedFields),
            syn::Fields::Unit => Err(TypeInfoError::UnitFields),
            syn::Fields::Unnamed(_) => {
                if item_ast.fields.len() == 1 {
                    let field = item_ast
                        .fields
                        .iter()
                        .next()
                        .expect("Field count was checked")
                        .clone();

                    Ok(Self {
                        name: item_ast.ident.clone(),
                        inner_type: field.ty,
                    })
                } else {
                    Err(TypeInfoError::InvalidFieldCount {
                        count: item_ast.fields.len(),
                    })
                }
            }
        }
    }
}

struct Stidgen<'a> {
    resolved_options: &'a options::Resolved,
    type_info: IdTypeInfo,
    item_ast: &'a syn::ItemStruct,
}

impl<'a> Stidgen<'a> {
    pub fn new(
        item_ast: &'a syn::ItemStruct,
        resolved_options: &'a options::Resolved,
        type_info: IdTypeInfo,
    ) -> Self {
        Self {
            resolved_options,
            item_ast,
            type_info,
        }
    }

    pub fn to_tokens(&self) -> TokenStream2 {
        let name = &(self.type_info.name);
        let inner_type = &(self.type_info.inner_type);
        let options = self.resolved_options;
        let item_ast = self.item_ast;

        let clone = add_impl_if_enabled!(options.clone, impls::clone(name));
        let hash = add_impl_if_enabled!(options.hash, impls::hash(name));
        let eq = add_impl_if_enabled!(options.eq, impls::eq(name));
        let partial_eq = add_impl_if_enabled!(options.partial_eq, impls::partial_eq(name));
        let ord = add_impl_if_enabled!(options.ord, impls::ord(name));
        let partial_ord = add_impl_if_enabled!(options.partial_ord, impls::partial_ord(name));
        let display = add_impl_if_enabled!(options.display, impls::display(name));
        let to_string = add_impl_if_enabled!(options.to_string, impls::to_string(name, inner_type));
        let debug = add_impl_if_enabled!(options.debug, impls::debug(name));
        let as_bytes = add_impl_if_enabled!(options.as_bytes, impls::as_bytes(name));
        let into_inner =
            add_impl_if_enabled!(options.into_inner, impls::into_inner(name, inner_type));
        let new = add_impl_if_enabled!(options.new, impls::new(name, inner_type));
        let as_ref = add_impl_if_enabled!(options.as_ref, impls::as_ref(name));
        let borrow = add_impl_if_enabled!(options.borrow_string, impls::borrow_string(name));
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
            #to_string
            #debug
            #as_ref
            #borrow
            #as_bytes
            #as_str
        }
    }
}

fn get_options(_attr_ast: &syn::AttributeArgs, id_type_info: &IdTypeInfo) -> options::Resolved {
    let user_options = options::Options::default();
    let defaults = known_types::get_defaults(&id_type_info.inner_type);
    let options = user_options.resolve(defaults);
    options
}

fn impl_id_type(attr_ast: &syn::AttributeArgs, item_ast: &syn::ItemStruct) -> TokenStream {
    let id_type_info = IdTypeInfo::new(item_ast).unwrap(); // TODO: We resolve that twice...
    let options = get_options(attr_ast, &id_type_info);
    let gen = Stidgen::new(item_ast, &options, id_type_info);

    gen.to_tokens().into()
}

#[proc_macro_attribute]
pub fn string_id(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let attr_ast = parse_macro_input!(attr as syn::AttributeArgs);
    let item_ast = parse_macro_input!(item as syn::ItemStruct);

    // Build the trait implementation
    impl_id_type(&attr_ast, &item_ast)
}
