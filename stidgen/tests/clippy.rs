#![allow(clippy::blanket_clippy_restriction_lints)]
#![warn(clippy::all, clippy::pedantic)]
// The following clippy entries are essentialy 'clippy::restriction' but it emit a warning :-(
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::else_if_without_else,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_arithmetic,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::inline_asm_x86_att_syntax,
    clippy::inline_asm_x86_intel_syntax,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::modulo_arithmetic,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::pattern_type_mismatch,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,
    clippy::wrong_pub_self_convention
)]

use stidgen::string_id;

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
