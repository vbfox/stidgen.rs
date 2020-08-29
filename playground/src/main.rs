use stidgen::string_id;
use std::borrow::Borrow;
use std::convert::AsRef;

#[string_id]
struct FooId(String);

fn with_str(_s: &str) {

}

fn main() {
    let id = FooId::new("Foo");
    let idstr = FooId::new("Bar");
    let idstr2 = FooId::new("Baz");
    let s: String = idstr2.into();
    with_str(id.borrow());
    with_str(id.as_ref());

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
