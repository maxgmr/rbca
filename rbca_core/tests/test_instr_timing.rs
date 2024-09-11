#![cfg(test)]

mod common;

use common::blargg_test_common;

#[test]
#[ignore]
fn test_instr_timing() {
    blargg_test_common(
        "INSTR_TIMING",
        "../roms/gb-test-roms/instr_timing/instr_timing.gb",
    );
}
