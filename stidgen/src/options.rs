#[derive(Debug, Clone, Copy)]
pub enum KnownTypes {
    String,
}

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
    pub debug: Option<bool>,
    pub as_bytes: Option<bool>,
    pub borrow: Option<bool>,
    pub as_ref: Option<bool>,
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
            debug: None,
            as_bytes: None,
            borrow: None,
            as_ref: None,
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
        resolve_one!(self, resolved, debug);
        resolve_one!(self, resolved, as_bytes);
        resolve_one!(self, resolved, borrow);
        resolve_one!(self, resolved, as_ref);

        resolved
    }

    pub fn resolve_for(&self, known_type: Option<KnownTypes>) -> Resolved {
        let defaults = get_defaults(known_type);
        self.resolve(defaults)
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
    pub debug: bool,
    pub as_bytes: bool,
    pub borrow: bool,
    pub as_ref: bool,
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
};

fn get_defaults(known_type: Option<KnownTypes>) -> &'static Resolved {
    match known_type {
        Some(KnownTypes::String) => &STRING_DEFAULTS,
        None => &ANY_DEFAULTS,
    }
}
