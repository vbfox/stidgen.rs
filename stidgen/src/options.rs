use std::collections::HashMap;

use once_cell::sync::Lazy;
use proc_macro2::Span;
use syn::{spanned::Spanned, Path, PathSegment};

/// External facing options
#[derive(Debug, Clone)]
pub struct Options {
    pub apply_defaults: bool,
    pub clone: Option<bool>,
    pub hash: Option<bool>,
    pub partial_eq: Option<bool>,
    pub eq: Option<bool>,
    pub partial_ord: Option<bool>,
    pub ord: Option<bool>,
    pub display: Option<bool>,
    to_string: Option<bool>,
    pub debug: Option<bool>,
    pub as_bytes: Option<bool>,
    pub borrow_string: Option<bool>,
    pub as_ref: Option<bool>,
    pub into_inner: Option<bool>,
    pub new: Option<bool>,
    pub as_str: Option<bool>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            apply_defaults: true,
            clone: None,
            hash: None,
            partial_eq: None,
            eq: None,
            partial_ord: None,
            ord: None,
            display: None,
            to_string: None,
            debug: None,
            as_bytes: None,
            borrow_string: None,
            as_ref: None,
            into_inner: None,
            new: None,
            as_str: None,
        }
    }
}

macro_rules! resolve_one {
    ( $self:expr, $resolved:expr, $x:ident ) => {{
        match $self.$x {
            Some(value) => $resolved.$x = value,
            None => {}
        }
    }};
}

impl Options {
    pub fn resolve(&self, defaults: &Resolved) -> Resolved {
        let mut resolved = if self.apply_defaults {
            defaults.clone()
        } else {
            Resolved::default()
        };

        resolve_one!(self, resolved, clone);
        resolve_one!(self, resolved, hash);
        resolve_one!(self, resolved, partial_eq);
        resolve_one!(self, resolved, eq);
        resolve_one!(self, resolved, partial_ord);
        resolve_one!(self, resolved, ord);
        resolve_one!(self, resolved, display);
        resolve_one!(self, resolved, to_string);
        resolve_one!(self, resolved, debug);
        resolve_one!(self, resolved, as_bytes);
        resolve_one!(self, resolved, borrow_string);
        resolve_one!(self, resolved, as_ref);
        resolve_one!(self, resolved, into_inner);
        resolve_one!(self, resolved, new);
        resolve_one!(self, resolved, as_str);

        resolved
    }
}

/// Options for the generator resolved from the passed in options
/// and the type of ID
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Default)]
pub struct Resolved {
    pub clone: bool,
    pub hash: bool,
    pub partial_eq: bool,
    pub eq: bool,
    pub partial_ord: bool,
    pub ord: bool,
    pub display: bool,
    pub to_string: bool,
    pub debug: bool,
    pub as_bytes: bool,
    pub borrow_string: bool,
    pub as_ref: bool,
    pub into_inner: bool,
    pub new: bool,
    pub as_str: bool,
}

#[derive(Clone, Copy)]
enum OptionArg {
    Defaults,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Display,
    ToString,
    Debug,
    AsBytes,
    BorrowString,
    AsRef,
    IntoInner,
    New,
    AsStr,
}

static OPTION_NAMES: Lazy<HashMap<String, (OptionArg, bool)>> = Lazy::new(|| {
    vec![
        ("Defaults", OptionArg::Defaults),
        ("Clone", OptionArg::Clone),
        ("Hash", OptionArg::Hash),
        ("Eq", OptionArg::Eq),
        ("PartialEq", OptionArg::PartialEq),
        ("Ord", OptionArg::Ord),
        ("PartialOrd", OptionArg::PartialOrd),
        ("Display", OptionArg::Display),
        ("ToString", OptionArg::ToString),
        ("Debug", OptionArg::Debug),
        ("AsBytes", OptionArg::AsBytes),
        ("BorrowString", OptionArg::BorrowString),
        ("AsRef", OptionArg::AsRef),
        ("IntoInner", OptionArg::IntoInner),
        ("New", OptionArg::New),
        ("AsStr", OptionArg::AsStr),
    ]
    .into_iter()
    .flat_map(|(s, a)| vec![(s.to_string(), (a, false)), (format!("No{}", s), (a, true))])
    .collect::<HashMap<_, _>>()
});

struct NegatableOptionArg {
    option: OptionArg,
    negated: bool,
    _span: Span,
}

impl NegatableOptionArg {
    fn new(option: OptionArg, negated: bool, span: Span) -> NegatableOptionArg {
        NegatableOptionArg {
            option,
            negated,
            _span: span,
        }
    }

    fn apply(&self, options: &mut Options) {
        match self.option {
            OptionArg::Defaults => options.apply_defaults = !self.negated,
            OptionArg::Clone => options.clone = Some(!self.negated),
            OptionArg::Hash => options.hash = Some(!self.negated),
            OptionArg::Eq => options.eq = Some(!self.negated),
            OptionArg::PartialEq => options.partial_eq = Some(!self.negated),
            OptionArg::Ord => options.ord = Some(!self.negated),
            OptionArg::PartialOrd => options.partial_ord = Some(!self.negated),
            OptionArg::Display => options.display = Some(!self.negated),
            OptionArg::ToString => options.to_string = Some(!self.negated),
            OptionArg::Debug => options.debug = Some(!self.negated),
            OptionArg::AsBytes => options.as_bytes = Some(!self.negated),
            OptionArg::BorrowString => options.borrow_string = Some(!self.negated),
            OptionArg::AsRef => options.as_ref = Some(!self.negated),
            OptionArg::IntoInner => options.into_inner = Some(!self.negated),
            OptionArg::New => options.new = Some(!self.negated),
            OptionArg::AsStr => options.as_str = Some(!self.negated),
        }
    }
}

fn parse_arg(segment: &PathSegment) -> syn::Result<NegatableOptionArg> {
    let span = segment.span();
    let s = segment.ident.to_string();
    match OPTION_NAMES.get(&s) {
        Some((option, negated)) => Ok(NegatableOptionArg::new(*option, *negated, span)),
        None => Err(syn::Error::new(span, format!("Unknown option: {}", s))),
    }
}

fn parse_args_path(path: &Path, span: Span) -> syn::Result<NegatableOptionArg> {
    if path.segments.len() == 1 {
        let segment = path.segments.first().expect("No first segment");
        parse_arg(segment)
    } else {
        Err(syn::Error::new(span, ""))
    }
}

fn parse_args_parts(args: &syn::AttributeArgs) -> syn::Result<Vec<NegatableOptionArg>> {
    args.iter()
        .map(|nested_meta| match nested_meta {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(p) => parse_args_path(p, meta.span()),
                syn::Meta::List(_) => Err(syn::Error::new(nested_meta.span(), "list")),
                syn::Meta::NameValue(_) => Err(syn::Error::new(nested_meta.span(), "kv")),
            },
            syn::NestedMeta::Lit(_) => Err(syn::Error::new(nested_meta.span(), "lit")),
        })
        .collect()
}

pub fn parse(attr_ast: &syn::AttributeArgs) -> syn::Result<Options> {
    let mut options = Options::default();

    let parts = parse_args_parts(attr_ast)?;

    for part in parts {
        part.apply(&mut options);
    }

    Ok(options)
}

#[cfg(test)]
pub mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use super::Options;

    fn parse_for_tests(token_stream: TokenStream) -> syn::Result<Options> {
        let (attr_ast, _) = crate::tests::parse_for_tests(token_stream);
        super::parse(&attr_ast)
    }

    #[test]
    fn empty() {
        let r = parse_for_tests(quote! {
            #[id]
            pub struct Id(String);
        });
        assert_eq!(r.is_ok(), true);
        let value = r.unwrap();
        assert_eq!(value.apply_defaults, true);
        assert_eq!(value.clone, None);
        assert_eq!(value.hash, None);
        assert_eq!(value.eq, None);
    }

    #[test]
    fn invalid_option() {
        let r = parse_for_tests(quote! {
            #[id(XXX)]
            pub struct Id(String);
        });
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn valid_options() {
        let r = parse_for_tests(quote! {
            #[id(Clone, Hash, NoEq)]
            pub struct Id(String);
        });
        assert_eq!(r.is_ok(), true);
        let value = r.unwrap();
        assert_eq!(value.clone, Some(true));
        assert_eq!(value.hash, Some(true));
        assert_eq!(value.eq, Some(false));
    }
}
