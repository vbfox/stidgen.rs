use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, Ident, Type};

use crate::type_match::TYPE_STRING;

pub fn clone(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::std::clone::Clone for #name {
            #[inline]
            fn clone(&self) -> Self {
                #name(self.0.clone())
            }
        }
    }
}

pub fn hash(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::std::hash::Hash for #name {
            #[inline]
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    }
}

pub fn partial_eq(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::std::cmp::PartialEq for #name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
    }
}

pub fn eq(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::std::cmp::Eq for #name {}
    }
}

pub fn ord(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::std::cmp::Ord for #name {
            #[inline]
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }
    }
}

pub fn partial_ord(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::std::cmp::PartialOrd for #name {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
    }
}

pub fn display(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::std::fmt::Display for #name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
    }
}

pub fn to_string(name: &Ident, inner_type: &Type) -> TokenStream2 {
    if &*TYPE_STRING == inner_type {
        quote! {
            #[automatically_derived]
            impl #name {
                /// Converts ID to a [String].
                #[inline]
                pub fn to_string(&self) -> String {
                    self.0.clone()
                }
            }
        }
    } else {
        quote! {
            #[automatically_derived]
            impl #name {
                /// Converts ID to a [String].
                #[inline]
                pub fn to_string(&self) -> String {
                    use ::std::fmt::Write;
                    let mut buf = String::new();
                    buf.write_fmt(format_args!("{}", self))
                        .expect("a Display implementation returned an error unexpectedly");
                    buf
                }
            }
        }
    }
}

pub fn debug(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!(#name))
                 .field(&self.0)
                 .finish()
            }
        }
    }
}

pub fn as_bytes(name: &Ident) -> TokenStream2 {
    quote! {
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
    }
}

pub fn into_inner(name: &Ident, inner_type: &Type) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl #name {
            #[inline]
            pub fn into_inner(self) -> #inner_type {
                self.0
            }
        }

        #[automatically_derived]
        impl ::std::convert::From<#name> for #inner_type {
            #[inline]
            fn from(value: #name) -> #inner_type {
                value.0
            }
        }
    }
}

pub fn new(name: &Ident, inner_type: &Type) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl #name {
            #[inline]
            pub fn new<S: Into<#inner_type>>(s: S) -> #name {
                #name(s.into())
            }
        }
    }
}

pub fn borrow_string(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::std::borrow::Borrow<str> for #name {
            #[inline]
            fn borrow(&self) -> &str {
                &self.0
            }
        }
    }
}

pub fn as_ref(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl ::std::convert::AsRef<str> for #name {
            #[inline]
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    }
}

pub fn as_str(name: &Ident) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl #name {
            /// Extracts a string slice containing the entire ID.
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    }
}
