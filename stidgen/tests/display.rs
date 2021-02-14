use std::fmt::Display;
use stidgen::string_id;

#[test]
fn string_display_impl() {
    #[string_id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: Display);
}

#[test]
fn string_display_string() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    assert_eq!("bar", format!("{}", id));
}

#[test]
fn i32_display_impl() {
    #[string_id]
    struct FooId(i32);

    static_assertions::assert_impl_all!(FooId: Display);
}

#[test]
fn i32_display_string() {
    #[string_id]
    struct FooId(i32);

    let id = FooId::new(42);
    assert_eq!("42", format!("{}", id));
}
