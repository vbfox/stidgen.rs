use std::borrow::Borrow;
use std::convert::AsRef;
use stidgen::string_id;

#[string_id(NoDefaults, Format)]
pub struct FooStringId(String);

#[string_id(NoDefaults, Format)]
pub struct FooI32Id(i32);

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
}
