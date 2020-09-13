use std::cmp::Ordering;
use stidgen::string_id;

#[string_id]
struct FooId(String);

#[test]
fn eq_impl() {
    static_assertions::assert_impl_all!(FooId: Eq);
    static_assertions::assert_impl_all!(FooId: PartialEq);
}

#[test]
fn eq() {
    let id = FooId::new("bar");
    assert_eq!(FooId::new("bar"), id);
    assert_ne!(FooId::new("baz"), id);
    assert!(FooId::new("bar") == id, true);
    assert!(FooId::new("baz") != id, true);
}

#[test]
fn ord_impl() {
    static_assertions::assert_impl_all!(FooId: Ord);
    static_assertions::assert_impl_all!(FooId: PartialOrd);
}

#[test]
fn partial_ord() {
    let id = FooId::new("bar");
    assert_eq!(FooId::new("bar").partial_cmp(&id), Some(Ordering::Equal));
    assert_eq!(FooId::new("car").partial_cmp(&id), Some(Ordering::Greater));
    assert_eq!(FooId::new("aar").partial_cmp(&id), Some(Ordering::Less));
}

#[test]
fn ord() {
    let id = FooId::new("bar");
    assert_eq!(FooId::new("bar").cmp(&id), Ordering::Equal);
    assert_eq!(FooId::new("car").cmp(&id), Ordering::Greater);
    assert_eq!(FooId::new("aar").cmp(&id), Ordering::Less);

    assert!(FooId::new("car") > id);
    assert!(FooId::new("aar") < id);
}
