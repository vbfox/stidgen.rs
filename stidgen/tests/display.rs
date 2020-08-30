use std::fmt::Display;
use stidgen::string_id;

#[test]
fn debug_impl() {
    #[string_id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: Display);
}

#[test]
fn display_string() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    assert_eq!("bar", format!("{}", id));
}
