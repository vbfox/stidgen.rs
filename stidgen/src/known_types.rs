use crate::options::Resolved;
use once_cell::sync::Lazy;
use syn::{parse::ParseBuffer, Type, TypeParen};

#[derive(Debug, Clone, Copy)]
pub enum KnownTypes {
    String,
}

static ANY_DEFAULTS: Resolved = Resolved {
    clone: false,
    hash: false,
    partial_eq: false,
    eq: false,
    partial_ord: false,
    ord: false,
    display: false,
    debug: false,
    as_bytes: false,
    borrow: false,
    as_ref: false,
    into_inner: false,
    new: false,
    as_str: false,
};

static STRING_DEFAULTS: Resolved = Resolved {
    clone: true,
    hash: true,
    partial_eq: true,
    eq: true,
    partial_ord: true,
    ord: true,
    display: true,
    debug: true,
    as_bytes: true,
    borrow: true,
    as_ref: true,
    into_inner: true,
    new: true,
    as_str: true,
};

struct KnownTypeInfo {
    pub known_type: KnownTypes,
    pub default_options: Resolved,
    pub types: Vec<syn::Type>,
}

impl KnownTypeInfo {
    pub fn new(known_type: KnownTypes, default_options: Resolved, parseable_types: Vec<&str>) -> KnownTypeInfo {
        let types = parseable_types
            .into_iter()
            .map(|t| syn::parse_str(t).expect("Hardcoded types should parse"))
            .collect::<Vec<syn::Type>>();

        #[cfg(debug_assertions)]
        for ty in types.iter() {
            if try_get_path_type(&ty) != Some(&ty) {
                panic!("Only Path types should be registered as known")
            }
        }

        KnownTypeInfo { known_type, default_options, types }
    }
}


static KNOWN_TYPE_INFOS: Lazy<Vec<KnownTypeInfo>> = Lazy::new(|| {
    let mut result: Vec<KnownTypeInfo> = Vec::new();

    result.push(
        KnownTypeInfo::new(
            KnownTypes::String,
            STRING_DEFAULTS,
            vec!("String", "std::string::String", "::std::string::String")
        )
    );

    result
});

//let t: Type = syn::parse_str("std::collections::HashMap<String, Value>")?;

/// Get the type if it is a `Type::Path`, extract the `Type::Path` if wrapped in `Type::Paren`, `None` otherwise.
fn try_get_path_type(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Paren(paren) => try_get_path_type(&paren.elem),
        Type::Path(_) => {
            Some(&ty)
        },
        _ => None,
    }
}

impl KnownTypeInfo {
    pub fn from_type(ty: &Type) -> Option<&KnownTypeInfo> {
        let path_type = try_get_path_type(ty)?;
        KNOWN_TYPE_INFOS.iter().find(|ti| {
            ti.types.iter().find(|t| *t == path_type).is_some()
        })
    }
}

pub fn get_defaults(known_type: Option<KnownTypes>) -> &'static Resolved {
    match known_type {
        Some(KnownTypes::String) => &STRING_DEFAULTS,
        None => &ANY_DEFAULTS,
    }
}
