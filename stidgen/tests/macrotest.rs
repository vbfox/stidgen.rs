#[test]
pub fn macrotest_pass() {
    macrotest::expand("tests/expand/*.rs");
}
