use stidgen::string_id;

#[test]
fn display_string() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    assert_eq!("bar", format!("{}", id));
}