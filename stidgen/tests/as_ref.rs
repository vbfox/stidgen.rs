use std::convert::AsRef;
use stidgen::string_id;

#[test]
fn as_ref_impl() {
    #[string_id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: AsRef<str>);
}

#[test]
fn as_ref_string() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    let b = AsRef::<str>::as_ref(&id);
    assert_eq!("bar", b);
}
