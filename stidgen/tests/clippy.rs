#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic
)]

use stidgen::string_id;

#[string_id]
struct FooId(String);
