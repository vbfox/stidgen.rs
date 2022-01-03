use stidgen::id;
#[repr(transparent)]
pub struct Id(String);
#[automatically_derived]
impl Id {
    #[inline]
    pub fn new<S: Into<String>>(s: S) -> Id {
        Id(s.into())
    }
}
#[automatically_derived]
impl Id {
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}
#[automatically_derived]
impl ::std::convert::From<Id> for String {
    #[inline]
    fn from(value: Id) -> String {
        value.0
    }
}
#[automatically_derived]
impl ::std::clone::Clone for Id {
    #[inline]
    fn clone(&self) -> Self {
        Id(self.0.clone())
    }
}
#[automatically_derived]
impl ::std::hash::Hash for Id {
    #[inline]
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
#[automatically_derived]
impl ::std::cmp::Eq for Id {}
#[automatically_derived]
impl ::std::cmp::PartialEq for Id {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::std::cmp::Ord for Id {
    #[inline]
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[automatically_derived]
impl ::std::cmp::PartialOrd for Id {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
#[automatically_derived]
impl ::std::fmt::Display for Id {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self.0, f)
    }
}
#[automatically_derived]
impl Id {
    /// Converts ID to a [String].
    #[inline]
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
#[automatically_derived]
impl ::std::fmt::Debug for Id {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_tuple("Id").field(&self.0).finish()
    }
}
#[automatically_derived]
impl ::std::convert::AsRef<str> for Id {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[automatically_derived]
impl ::std::borrow::Borrow<str> for Id {
    #[inline]
    fn borrow(&self) -> &str {
        &self.0
    }
}
#[automatically_derived]
impl ::std::convert::AsRef<[u8]> for Id {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        ::std::convert::AsRef::<[u8]>::as_ref(&self.0)
    }
}
#[automatically_derived]
impl Id {
    /// Returns a byte slice of this ID's contents.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        ::std::convert::AsRef::<[u8]>::as_ref(&self.0)
    }
}
#[automatically_derived]
impl Id {
    /// Extracts a string slice containing the entire ID.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
