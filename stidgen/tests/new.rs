use stidgen::string_id;

#[test]
fn new_string() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar".to_string());
    assert_eq!("bar", id.as_str());
}

#[test]
fn new_str() {
    #[string_id]
    struct FooId(String);

    let id = FooId::new("bar");
    assert_eq!("bar", id.as_str());
}

#[test]
fn new_into_string() {
    struct IntoString {}

    impl Into<String> for IntoString {
        fn into(self) -> String {
            "bar".to_string()
        }
    }

    #[string_id]
    struct FooId(String);

    let id = FooId::new(IntoString {});
    assert_eq!("bar", id.as_str());
}

// ----

#[test]
fn new_i32() {
    #[string_id]
    struct FooId(i32);

    let id = FooId::new(42);
    assert_eq!(42, id.into_inner());
}

#[test]
fn new_into_i32() {
    struct IntoNumber {}

    impl Into<i32> for IntoNumber {
        fn into(self) -> i32 {
            42
        }
    }

    #[string_id]
    struct FooId(i32);

    let id = FooId::new(IntoNumber {});
    assert_eq!(42, id.into_inner());
}
