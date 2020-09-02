use stidgen::string_id;

#[test]
fn clone_impl() {
    #[string_id]
    struct FooId(String);

    static_assertions::assert_impl_all!(FooId: Clone);
}

#[test]
fn clone_id() {
    #[string_id]
    struct FooId(String);

    fn consume(id: FooId) {
        assert_eq!(id.as_str(), "bar");
    }

    let id = FooId::new("bar".to_string());
    let id2 = id.clone();
    consume(id);
    consume(id2);
}
