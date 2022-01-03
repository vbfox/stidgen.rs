use stidgen::id;
#[repr(transparent)]
pub struct Id(String);
#[automatically_derived]
impl ::std::clone::Clone for Id {
    #[inline]
    fn clone(&self) -> Self {
        Id(self.0.clone())
    }
}
#[automatically_derived]
impl ::std::cmp::PartialEq for Id {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::std::cmp::PartialOrd for Id {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
