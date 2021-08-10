use once_cell::sync::Lazy;
use syn::Type;

pub struct MatchableType {
    pub types: Vec<String>,
}

/// Get the type if it is a `Type::Path`, extract the `Type::Path` if wrapped in `Type::Paren`, `None` otherwise.
fn try_get_path_type(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Paren(paren) => try_get_path_type(&paren.elem),
        Type::Path(_) => Some(ty),
        _ => None,
    }
}

impl MatchableType {
    pub fn new(parseable_types: Vec<&str>) -> MatchableType {
        #[cfg(debug_assertions)]
        {
            let parsed_types = parseable_types
                .iter()
                .map(|t| syn::parse_str(t).expect("Hardcoded types should parse"))
                .collect::<Vec<syn::Type>>();

            for ty in &parsed_types {
                match try_get_path_type(ty) {
                    Some(_) => {}
                    None => panic!("Only Path types should be registered as known"),
                }
            }
        }

        let types = parseable_types
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        MatchableType { types }
    }

    fn types_iter(&self) -> impl Iterator<Item = Type> + '_ {
        self.types
            .iter()
            .map(|t| syn::parse_str(t).expect("Hardcoded types should parse"))
    }
}

impl PartialEq<Type> for MatchableType {
    fn eq(&self, other: &Type) -> bool {
        self.types_iter().any(|m| m == *other)
    }
}

pub static TYPE_STRING: Lazy<MatchableType> = Lazy::new(|| {
    MatchableType::new(vec![
        "String",
        "std::string::String",
        "::std::string::String",
    ])
});
pub static TYPE_I8: Lazy<MatchableType> = Lazy::new(|| MatchableType::new(vec!["i8", "std::i8"]));
pub static TYPE_U8: Lazy<MatchableType> = Lazy::new(|| MatchableType::new(vec!["u8", "std::u8"]));
pub static TYPE_I16: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["i16", "std::i16"]));
pub static TYPE_U16: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["u16", "std::u16"]));
pub static TYPE_I32: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["i32", "std::i32"]));
pub static TYPE_U32: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["u32", "std::u32"]));
pub static TYPE_I64: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["i64", "std::i64"]));
pub static TYPE_U64: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["u64", "std::u64"]));
pub static TYPE_I128: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["i128", "std::i128"]));
pub static TYPE_U128: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["u128", "std::u128"]));
pub static TYPE_ISIZE: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["isize", "std::isize"]));
pub static TYPE_USIZE: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["usize", "std::usize"]));
pub static TYPE_UUID: Lazy<MatchableType> =
    Lazy::new(|| MatchableType::new(vec!["Uuid", "uuid::Uuid"]));
