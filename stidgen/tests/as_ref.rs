use std::convert::AsRef;
use stidgen::string_id;

#[test]
fn string_as_ref_str_impl() {
    #[string_id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: AsRef<str>);
}

#[test]
fn string_as_ref_u8_impl() {
    #[string_id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: AsRef<[u8]>);
}

#[test]
fn string_as_ref_str() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    let b = AsRef::<str>::as_ref(&id);
    assert_eq!("bar", b);
}

#[test]
fn string_as_ref_u8() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    let b = AsRef::<[u8]>::as_ref(&id);
    assert_eq!(&[98, 97, 114], b);
}

#[test]
fn string_as_bytes() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    let b = id.as_bytes();
    assert_eq!(&[98, 97, 114], b);
}
