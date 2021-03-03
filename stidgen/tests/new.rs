use stidgen::id;

#[test]
fn new_string() {
    #[id]
    struct FooId(String);

    let id = FooId::new("bar".to_string());
    assert_eq!("bar", id.as_str());
}

#[test]
fn new_str() {
    #[id]
    struct FooId(String);

    let id = FooId::new("bar");
    assert_eq!("bar", id.as_str());
}

#[test]
fn new_into_string() {
    struct IntoString {}

    impl From<IntoString> for String {
        fn from(_: IntoString) -> String {
            "bar".to_string()
        }
    }

    #[id]
    struct FooId(String);

    let id = FooId::new(IntoString {});
    assert_eq!("bar", id.as_str());
}

// ----

#[test]
fn new_i32() {
    #[id]
    struct FooId(i32);

    let id = FooId::new(42);
    assert_eq!(42, id.into_inner());
}

#[test]
fn new_into_i32() {
    struct IntoNumber {}

    impl From<IntoNumber> for i32 {
        fn from(_: IntoNumber) -> i32 {
            42
        }
    }

    #[id]
    struct FooId(i32);

    let id = FooId::new(IntoNumber {});
    assert_eq!(42, id.into_inner());
}
