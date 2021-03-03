use std::borrow::Borrow;
use std::convert::AsRef;
use stidgen::id;
use uuid::Uuid;

#[id]
pub struct FooStringId(String);

#[id]
pub struct FooI32Id(i32);

#[id]
pub struct FooGuidId(Uuid);

fn with_str(_s: &str) {}

fn main() {
    let id = FooStringId::new("Foo");
    let idstr = FooStringId::new("Bar");
    let idstr2 = FooStringId::new("Baz");
    let s: String = idstr2.into();
    with_str(id.borrow());
    with_str(id.as_ref());

    println!(
        "format {}\ndebug {:?}\nas_str {}\nto_string {}\ninto_string {}\ninto {}",
        id,
        id,
        id.as_str(),
        id.to_string(),
        idstr.into_inner(),
        s
    );

    println!("---");
    let xi = FooI32Id::new(42);
    println!(
        "format {}\ndebug {:?}\nto_string {}",
        xi,
        xi,
        xi.to_string()
    );

    println!("---");
    let xi = FooGuidId::new(Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap());
    println!(
        "format {}\ndebug {:?}\nto_string {}",
        xi,
        xi,
        xi.to_string()
    );
}
