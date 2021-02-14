use stidgen::string_id;

mod std {
    pub mod cmp {
        pub trait PartialEq {}
    }
}

// This would fail if the macro simply used 'std'
#[string_id]
struct FooId(String);

#[string_id]
struct FooIdU8(u8);

#[string_id]
struct FooIdI8(i8);

#[string_id]
struct FooIdU16(u16);

#[string_id]
struct FooIdI16(i16);

#[string_id]
struct FooIdU32(u32);

#[string_id]
struct FooIdI32(i32);

#[string_id]
struct FooIdU64(u64);

#[string_id]
struct FooIdI64(i64);

#[string_id]
struct FooIdU128(u128);

#[string_id]
struct FooIdI128(i128);

#[string_id]
struct FooIdUSize(usize);

#[string_id]
struct FooIdISize(isize);
