use stidgen::id;

#[id]
struct FooId(String);

#[test]
fn test_send() {
    fn assert_send<T: Send>() {}
    assert_send::<FooId>();
}

#[test]
fn test_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<FooId>();
}
