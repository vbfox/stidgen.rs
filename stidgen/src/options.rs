/// External facing options
#[derive(Debug, Clone)]
pub struct Options {
    pub defaults: Option<bool>,
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
            defaults: None,
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
        let mut resolved = defaults.clone();

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
#[derive(Debug, Clone)]
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

pub fn parse_options(_attr_ast: &syn::AttributeArgs) -> Options {
    Options::default()
}
