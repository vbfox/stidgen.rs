use stidgen::id;

#[id(NoDefaults, Clone, PartialEq, PartialOrd)]
pub struct Id(String);
