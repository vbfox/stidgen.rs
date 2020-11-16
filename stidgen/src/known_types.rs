use crate::options::Resolved;
use once_cell::sync::Lazy;
use syn::Type;

#[derive(Debug, Clone, Copy)]
enum KnownTypes {
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
    pub default_options: &'static Resolved,
    pub types: Vec<String>,
}

impl KnownTypeInfo {
    pub fn new(
        known_type: KnownTypes,
        default_options: &'static Resolved,
        parseable_types: Vec<&str>,
    ) -> KnownTypeInfo {
        #[cfg(debug_assertions)]
        {
            let parsed_types = parseable_types
                .iter()
                .map(|t| syn::parse_str(t).expect("Hardcoded types should parse"))
                .collect::<Vec<syn::Type>>();

            for ty in parsed_types.iter() {
                match try_get_path_type(&ty) {
                    Some(_) => {}
                    None => panic!("Only Path types should be registered as known"),
                }
            }
        }

        let types = parseable_types
            .into_iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>();

        KnownTypeInfo {
            known_type,
            default_options,
            types,
        }
    }

    fn types_iter(&self) -> impl Iterator<Item = Type> + '_ {
        self.types
            .iter()
            .map(|t| syn::parse_str(t).expect("Hardcoded types should parse"))
    }

    pub fn matches(&self, t: &Type) -> bool {
        self.types_iter().find(|m| *m == *t).is_some()
    }
}

fn build_defaults() -> Vec<KnownTypeInfo> {
    let mut result: Vec<KnownTypeInfo> = Vec::new();

    result.push(KnownTypeInfo::new(
        KnownTypes::String,
        &STRING_DEFAULTS,
        vec!["String", "std::string::String", "::std::string::String"],
    ));

    result
}

static KNOWN_TYPE_INFOS: Lazy<Vec<KnownTypeInfo>> = Lazy::new(|| build_defaults());

/// Get the type if it is a `Type::Path`, extract the `Type::Path` if wrapped in `Type::Paren`, `None` otherwise.
fn try_get_path_type(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Paren(paren) => try_get_path_type(&paren.elem),
        Type::Path(_) => Some(&ty),
        _ => None,
    }
}

impl KnownTypeInfo {
    pub fn from_type(ty: &Type) -> Option<&KnownTypeInfo> {
        let path_type = try_get_path_type(ty)?;
        KNOWN_TYPE_INFOS.iter().find(|ti| ti.matches(path_type))
    }
}

pub fn get_defaults(for_type: &Type) -> &'static Resolved {
    match KnownTypeInfo::from_type(for_type) {
        Some(known_type) => known_type.default_options,
        None => &ANY_DEFAULTS,
    }
}
