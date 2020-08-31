use stidgen::string_id;

#[test]
fn eq_impl() {
    #[string_id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: Eq);
    static_assertions::assert_impl_all!(FooId: PartialEq);
}

#[test]
fn eq() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    assert_eq!(FooId::new("bar"), id);
    assert_ne!(FooId::new("baz"), id);
    assert!(FooId::new("bar") == id, true);
    assert!(FooId::new("baz") != id, true);
}
