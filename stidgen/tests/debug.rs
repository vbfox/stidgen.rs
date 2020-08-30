use stidgen::string_id;

#[test]
fn debug_string() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    assert_eq!("FooId(\"bar\")", format!("{:?}", id));
}