mod util;
use util::*;

#[test]
fn u256() -> TestResult {
    test_default_generated_schema::<alloy_primitives::U256>("u256")
}

#[test]
fn address() -> TestResult {
    test_default_generated_schema::<alloy_primitives::Address>("address")
}
