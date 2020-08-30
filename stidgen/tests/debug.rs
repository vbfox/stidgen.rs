use std::fmt::Debug;
use stidgen::string_id;

#[test]
fn debug_impl() {
    #[string_id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: Debug);
}

#[test]
fn debug_string() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    assert_eq!("FooId(\"bar\")", format!("{:?}", id));
}
