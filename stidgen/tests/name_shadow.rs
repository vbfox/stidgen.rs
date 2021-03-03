use stidgen::id;

mod std {
    pub mod cmp {
        pub trait PartialEq {}
    }
}

// This would fail if the macro simply used 'std'
#[id]
struct FooId(String);

#[id]
struct FooIdU8(u8);

#[id]
struct FooIdI8(i8);

#[id]
struct FooIdU16(u16);

#[id]
struct FooIdI16(i16);

#[id]
struct FooIdU32(u32);

#[id]
struct FooIdI32(i32);

#[id]
struct FooIdU64(u64);

#[id]
struct FooIdI64(i64);

#[id]
struct FooIdU128(u128);

#[id]
struct FooIdI128(i128);

#[id]
struct FooIdUSize(usize);

#[id]
struct FooIdISize(isize);
