#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::ptr_arg)]

extern crate proc_macro;

/*

TODO:
- to_string should be separate from Display user-side too

*/

mod impls;
mod known_types;
mod options;
mod type_match;

use known_types::KnownTypeInfo;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::{self, parse_macro_input, Ident, Type};
use thiserror::Error;

macro_rules! add_impl_if_enabled {
    ( $option:expr, $impl:expr ) => {{
        if $option {
            $impl
        } else {
            proc_macro2::TokenStream::new()
        }
    }};
}

#[derive(Error, Debug)]
enum TypeInfoError {
    // struct Foo { x: i32 };
    #[error("expected an unamed field but found named ones")]
    NamedFields { span: Span },
    // struct Foo;
    #[error("expected an unamed field but found none")]
    UnitFields { span: Span },
    // struct Foo(i32, i32);
    #[error("expected a single field but found {count}")]
    InvalidFieldCount { count: usize, span: Span },
}

impl Spanned for TypeInfoError {
    fn span(&self) -> Span {
        *match self {
            TypeInfoError::NamedFields { span, .. }
            | TypeInfoError::UnitFields { span, .. }
            | TypeInfoError::InvalidFieldCount { span, .. } => span,
        }
    }
}

impl From<TypeInfoError> for syn::Error {
    fn from(err: TypeInfoError) -> Self {
        syn::Error::new(err.span(), err)
    }
}

struct IdTypeInfo {
    name: Ident,
    inner_type: Type,
    known_type: Option<KnownTypeInfo>,
}

impl IdTypeInfo {
    pub fn new(item_ast: &syn::ItemStruct) -> Result<Self, TypeInfoError> {
        match item_ast.fields {
            syn::Fields::Named(_) => Err(TypeInfoError::NamedFields {
                span: item_ast.fields.span(),
            }),
            syn::Fields::Unit => Err(TypeInfoError::UnitFields {
                span: item_ast.ident.span(),
            }),
            syn::Fields::Unnamed(_) => {
                if item_ast.fields.len() == 1 {
                    let field = item_ast
                        .fields
                        .iter()
                        .next()
                        .expect("Field count was checked")
                        .clone();

                    let inner_type = field.ty;
                    let known_type = KnownTypeInfo::from_type(&inner_type).cloned();

                    Ok(Self {
                        name: item_ast.ident.clone(),
                        inner_type,
                        known_type,
                    })
                } else {
                    Err(TypeInfoError::InvalidFieldCount {
                        count: item_ast.fields.len(),
                        span: item_ast.fields.span(),
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
            type_info,
            item_ast,
        }
    }

    pub fn to_tokens(&self) -> TokenStream2 {
        let name = &(self.type_info.name);
        let inner_type = &(self.type_info.inner_type);
        let known_type = self.type_info.known_type.as_ref().map(|k| k.known_type);
        let options = self.resolved_options;
        let item_ast = self.item_ast;

        let clone = add_impl_if_enabled!(options.clone, impls::clone(name));
        let hash = add_impl_if_enabled!(options.hash, impls::hash(name));
        let eq = add_impl_if_enabled!(options.eq, impls::eq(name));
        let partial_eq = add_impl_if_enabled!(options.partial_eq, impls::partial_eq(name));
        let ord = add_impl_if_enabled!(options.ord, impls::ord(name));
        let partial_ord = add_impl_if_enabled!(options.partial_ord, impls::partial_ord(name));
        let display = add_impl_if_enabled!(options.display, impls::display(name));
        let to_string = add_impl_if_enabled!(options.to_string, impls::to_string(name, known_type));
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

fn get_options(
    attr_ast: &syn::AttributeArgs,
    id_type_info: &IdTypeInfo,
) -> syn::Result<options::Resolved> {
    let user_options = options::parse(attr_ast)?;
    let defaults = known_types::get_default_options(id_type_info.known_type.as_ref());
    Ok(user_options.resolve(defaults))
}

fn impl_id_type(
    attr_ast: &syn::AttributeArgs,
    item_ast: &syn::ItemStruct,
) -> syn::Result<TokenStream2> {
    let id_type_info = IdTypeInfo::new(item_ast)?;
    let options = get_options(attr_ast, &id_type_info)?;
    let gen = Stidgen::new(item_ast, &options, id_type_info);

    Ok(gen.to_tokens())
}

#[proc_macro_attribute]
pub fn id(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let attr_ast = parse_macro_input!(attr as syn::AttributeArgs);
    // TODO: ? Attribute::parse_meta
    let item_ast = parse_macro_input!(item as syn::ItemStruct);

    // Build the trait implementation
    impl_id_type(&attr_ast, &item_ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[cfg(test)]
pub mod tests {
    use crate::IdTypeInfo;
    use proc_macro2::TokenStream;
    use quote::quote;

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn parse(token_stream: TokenStream) -> (syn::AttributeArgs, syn::ItemStruct) {
        let item_ast: syn::ItemStruct = syn::parse2(token_stream).unwrap();
        let attribute_meta = item_ast.attrs.get(0).unwrap().parse_meta().unwrap();

        let attr_ast = match attribute_meta {
            syn::Meta::List(meta_list) => meta_list.nested.into_iter().collect::<Vec<_>>(),
            syn::Meta::Path(_) => vec![],
            syn::Meta::NameValue(_) => panic!("Unknown attribute structure"),
        };

        (attr_ast, item_ast)
    }

    #[test]
    fn get_options_no_defaults() {
        let (attr_ast, item_ast) = parse(quote! {
            #[id(NoDefaults, Clone, PartialEq, PartialOrd)]
            pub struct Id(String);
        });
        let id_type = IdTypeInfo::new(&item_ast).unwrap();
        let resolved = super::get_options(&attr_ast, &id_type).unwrap();
        assert_eq!(resolved.eq, false);
        assert_eq!(resolved.clone, true);
        assert_eq!(resolved.partial_eq, true);
        assert_eq!(resolved.partial_ord, true);
    }

    #[test]
    fn get_options_defaults() {
        let (attr_ast, item_ast) = parse(quote! {
            #[id]
            pub struct Id(String);
        });
        let id_type = IdTypeInfo::new(&item_ast).unwrap();
        let resolved = super::get_options(&attr_ast, &id_type).unwrap();
        assert_eq!(resolved.eq, true);
        assert_eq!(resolved.clone, true);
        assert_eq!(resolved.partial_eq, true);
        assert_eq!(resolved.partial_ord, true);
    }
}
