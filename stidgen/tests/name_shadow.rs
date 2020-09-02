use stidgen::string_id;

mod std {
    pub mod cmp {
        pub trait PartialEq {}
    }
}

// This would fail if the macro simply used 'std'
#[string_id]
struct FooId(String);