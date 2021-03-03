use std::fmt::Debug;
use stidgen::id;

#[test]
fn debug_impl() {
    #[id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: Debug);
}

#[test]
fn debug_string() {
    #[id]
    struct FooId(String);

    let id = FooId::new("bar");
    assert_eq!("FooId(\"bar\")", format!("{:?}", id));
}
