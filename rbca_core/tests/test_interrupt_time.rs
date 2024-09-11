#![cfg(test)]

mod common;

use common::blargg_test_common;

#[test]
#[ignore]
fn test_interrupt_time() {
    blargg_test_common(
        "INTERRUPT_TIME",
        "../roms/gb-test-roms/interrupt_time/interrupt_time.gb",
    );
}
