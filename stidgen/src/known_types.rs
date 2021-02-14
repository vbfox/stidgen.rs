use crate::options::Resolved;
use once_cell::sync::Lazy;
use syn::Type;

#[derive(Debug, Clone, Copy)]
enum KnownTypes {
    String,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    ISize,
    USize,
}

static ANY_DEFAULTS: Resolved = Resolved {
    clone: false,
    hash: false,
    partial_eq: false,
    eq: false,
    partial_ord: false,
    ord: false,
    display: false,
    to_string: false,
    debug: false,
    as_bytes: false,
    borrow_string: false,
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
    to_string: true,
    debug: true,
    as_bytes: true,
    borrow_string: true,
    as_ref: true,
    into_inner: true,
    new: true,
    as_str: true,
};

static NUMBER_DEFAULTS: Resolved = Resolved {
    clone: true,
    hash: true,
    partial_eq: true,
    eq: true,
    partial_ord: true,
    ord: true,
    display: true,
    to_string: true,
    debug: true,
    as_bytes: false,
    borrow_string: false,
    as_ref: false,
    into_inner: true,
    new: true,
    as_str: false,
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

    result.push(KnownTypeInfo::new(
        KnownTypes::I8,
        &NUMBER_DEFAULTS,
        vec!["i8", "std::i8"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::U8,
        &NUMBER_DEFAULTS,
        vec!["u8", "std::u8"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::I16,
        &NUMBER_DEFAULTS,
        vec!["i16", "std::i16"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::U16,
        &NUMBER_DEFAULTS,
        vec!["u16", "std::u16"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::I32,
        &NUMBER_DEFAULTS,
        vec!["i32", "std::i32"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::U32,
        &NUMBER_DEFAULTS,
        vec!["u32", "std::u32"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::I64,
        &NUMBER_DEFAULTS,
        vec!["i64", "std::i64"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::U64,
        &NUMBER_DEFAULTS,
        vec!["u64", "std::u64"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::I128,
        &NUMBER_DEFAULTS,
        vec!["i128", "std::i128"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::U128,
        &NUMBER_DEFAULTS,
        vec!["u128", "std::u128"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::ISize,
        &NUMBER_DEFAULTS,
        vec!["isize", "std::isize"],
    ));

    result.push(KnownTypeInfo::new(
        KnownTypes::USize,
        &NUMBER_DEFAULTS,
        vec!["usize", "std::usize"],
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
