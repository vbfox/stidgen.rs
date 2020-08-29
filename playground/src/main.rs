use stidgen::HelloMacro;
use stidgen_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

/*
impl HelloMacro for Pancakes {
    fn hello_macro() {
    }
}*/

fn main() {
    Pancakes::hello_macro();
}