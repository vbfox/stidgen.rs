use std::borrow::Borrow;
use stidgen::id;

#[test]
fn borrow_impl() {
    #[id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: Borrow<str>);
}

#[test]
fn borrow_string() {
    #[id]
    struct FooId(String);

    let id = FooId::new("bar");
    let b = Borrow::<str>::borrow(&id);
    assert_eq!("bar", b);
}
