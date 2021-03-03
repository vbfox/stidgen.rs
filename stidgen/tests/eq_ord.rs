use std::cmp::Ordering;
use stidgen::id;

#[id]
struct TestStringId(String);

#[test]
fn string_eq_impl() {
    static_assertions::assert_impl_all!(TestStringId: Eq);
    static_assertions::assert_impl_all!(TestStringId: PartialEq);
}

#[test]
fn string_eq() {
    let id = TestStringId::new("bar");
    assert_eq!(TestStringId::new("bar"), id);
    assert_ne!(TestStringId::new("baz"), id);
    assert!(TestStringId::new("bar") == id);
    assert!(TestStringId::new("baz") != id);
}

#[test]
fn string_ord_impl() {
    static_assertions::assert_impl_all!(TestStringId: Ord);
    static_assertions::assert_impl_all!(TestStringId: PartialOrd);
}

#[test]
fn string_partial_ord() {
    let id = TestStringId::new("bar");
    assert_eq!(
        TestStringId::new("bar").partial_cmp(&id),
        Some(Ordering::Equal)
    );
    assert_eq!(
        TestStringId::new("car").partial_cmp(&id),
        Some(Ordering::Greater)
    );
    assert_eq!(
        TestStringId::new("aar").partial_cmp(&id),
        Some(Ordering::Less)
    );
}

#[test]
fn string_ord() {
    let id = TestStringId::new("bar");
    assert_eq!(TestStringId::new("bar").cmp(&id), Ordering::Equal);
    assert_eq!(TestStringId::new("car").cmp(&id), Ordering::Greater);
    assert_eq!(TestStringId::new("aar").cmp(&id), Ordering::Less);

    assert!(TestStringId::new("car") > id);
    assert!(TestStringId::new("aar") < id);
}

// ----

#[id]
struct TestI32Id(i32);

#[test]
fn i32_eq_impl() {
    static_assertions::assert_impl_all!(TestI32Id: Eq);
    static_assertions::assert_impl_all!(TestI32Id: PartialEq);
}

#[test]
fn i32_eq() {
    let id = TestI32Id::new(42);
    assert_eq!(TestI32Id::new(42), id);
    assert_ne!(TestI32Id::new(-1), id);
    assert!(TestI32Id::new(42) == id);
    assert!(TestI32Id::new(-1) != id);
}

#[test]
fn i32_ord_impl() {
    static_assertions::assert_impl_all!(TestI32Id: Ord);
    static_assertions::assert_impl_all!(TestI32Id: PartialOrd);
}

#[test]
fn i32_partial_ord() {
    let id = TestI32Id::new(42);
    assert_eq!(TestI32Id::new(42).partial_cmp(&id), Some(Ordering::Equal));
    assert_eq!(TestI32Id::new(43).partial_cmp(&id), Some(Ordering::Greater));
    assert_eq!(TestI32Id::new(41).partial_cmp(&id), Some(Ordering::Less));
}

#[test]
fn i32_ord() {
    let id = TestI32Id::new(42);
    assert_eq!(TestI32Id::new(42).cmp(&id), Ordering::Equal);
    assert_eq!(TestI32Id::new(43).cmp(&id), Ordering::Greater);
    assert_eq!(TestI32Id::new(41).cmp(&id), Ordering::Less);

    assert!(TestI32Id::new(43) > id);
    assert!(TestI32Id::new(41) < id);
}
