use crate::{
    options::Resolved,
    type_match::{
        MatchableType, TYPE_I128, TYPE_I16, TYPE_I32, TYPE_I64, TYPE_I8, TYPE_ISIZE, TYPE_STRING,
        TYPE_U128, TYPE_U16, TYPE_U32, TYPE_U64, TYPE_U8, TYPE_USIZE, TYPE_UUID,
    },
};
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
    Uuid,
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

static UUID_DEFAULTS: Resolved = Resolved {
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
    pub matchable: &'static MatchableType,
}

impl KnownTypeInfo {
    pub fn new(
        known_type: KnownTypes,
        default_options: &'static Resolved,
        matchable: &'static MatchableType,
    ) -> KnownTypeInfo {
        KnownTypeInfo {
            known_type,
            default_options,
            matchable,
        }
    }
}

impl PartialEq<Type> for KnownTypeInfo {
    fn eq(&self, other: &Type) -> bool {
        self.matchable == other
    }
}

static KNOWN_TYPE_INFOS: Lazy<Vec<KnownTypeInfo>> = Lazy::new(|| {
    vec![
        KnownTypeInfo::new(KnownTypes::String, &STRING_DEFAULTS, &*TYPE_STRING),
        KnownTypeInfo::new(KnownTypes::I8, &NUMBER_DEFAULTS, &*TYPE_I8),
        KnownTypeInfo::new(KnownTypes::U8, &NUMBER_DEFAULTS, &*TYPE_U8),
        KnownTypeInfo::new(KnownTypes::I16, &NUMBER_DEFAULTS, &*TYPE_I16),
        KnownTypeInfo::new(KnownTypes::U16, &NUMBER_DEFAULTS, &*TYPE_U16),
        KnownTypeInfo::new(KnownTypes::I32, &NUMBER_DEFAULTS, &*TYPE_I32),
        KnownTypeInfo::new(KnownTypes::U32, &NUMBER_DEFAULTS, &*TYPE_U32),
        KnownTypeInfo::new(KnownTypes::I64, &NUMBER_DEFAULTS, &*TYPE_I64),
        KnownTypeInfo::new(KnownTypes::U64, &NUMBER_DEFAULTS, &*TYPE_U64),
        KnownTypeInfo::new(KnownTypes::I128, &NUMBER_DEFAULTS, &*TYPE_I128),
        KnownTypeInfo::new(KnownTypes::U128, &NUMBER_DEFAULTS, &*TYPE_U128),
        KnownTypeInfo::new(KnownTypes::ISize, &NUMBER_DEFAULTS, &*TYPE_ISIZE),
        KnownTypeInfo::new(KnownTypes::USize, &NUMBER_DEFAULTS, &*TYPE_USIZE),
        KnownTypeInfo::new(KnownTypes::Uuid, &UUID_DEFAULTS, &*TYPE_UUID),
    ]
});

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
        KNOWN_TYPE_INFOS.iter().find(|ti| *ti == path_type)
    }
}

pub fn get_defaults(for_type: &Type) -> &'static Resolved {
    match KnownTypeInfo::from_type(for_type) {
        Some(known_type) => known_type.default_options,
        None => &ANY_DEFAULTS,
    }
}
