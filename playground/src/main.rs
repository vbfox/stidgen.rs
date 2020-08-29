use stidgen::string_id;

#[string_id]
struct FooId(String);

fn main() {
    let id = FooId::new("Foo");
    let idstr = FooId::new("Bar");
    let idstr2 = FooId::new("Baz");
    let s: String = idstr2.into();

    println!(
        "{}, {:?}, {}, {}, {}, {}",
        id,
        id,
        id.as_str(),
        id.to_string(),
        idstr.into_string(),
        s
    );
}
