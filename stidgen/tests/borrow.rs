use std::borrow::Borrow;
use stidgen::string_id;

#[test]
fn borrow_impl() {
    #[string_id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: Borrow<str>);
}

#[test]
fn borrow_string() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    let b = Borrow::<str>::borrow(&id);
    assert_eq!("bar", b);
}
