#![cfg(test)]

mod common;

use common::blargg_test_common;

#[test]
#[ignore]
fn test_cpu_all() {
    blargg_test_common("ALL", "../roms/gb-test-roms/cpu_instrs/cpu_instrs.gb")
}

// Example usage: cargo t 01 -- --nocapture --ignored
#[test]
#[ignore]
fn test_cpu_01() {
    blargg_test_common(
        "01",
        "../roms/gb-test-roms/cpu_instrs/individual/01-special.gb",
    );
}

#[test]
#[ignore]
fn test_cpu_02() {
    blargg_test_common(
        "02",
        "../roms/gb-test-roms/cpu_instrs/individual/02-interrupts.gb",
    );
}

#[test]
#[ignore]
fn test_cpu_03() {
    blargg_test_common(
        "03",
        "../roms/gb-test-roms/cpu_instrs/individual/03-op sp,hl.gb",
    );
}

#[test]
#[ignore]
fn test_cpu_04() {
    blargg_test_common(
        "04",
        "../roms/gb-test-roms/cpu_instrs/individual/04-op r,imm.gb",
    );
}

#[test]
#[ignore]
fn test_cpu_05() {
    blargg_test_common(
        "05",
        "../roms/gb-test-roms/cpu_instrs/individual/05-op rp.gb",
    );
}

#[test]
#[ignore]
fn test_cpu_06() {
    blargg_test_common(
        "06",
        "../roms/gb-test-roms/cpu_instrs/individual/06-ld r,r.gb",
    );
}

#[test]
#[ignore]
fn test_cpu_07() {
    blargg_test_common(
        "07",
        "../roms/gb-test-roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb",
    );
}

#[test]
#[ignore]
fn test_cpu_08() {
    blargg_test_common(
        "08",
        "../roms/gb-test-roms/cpu_instrs/individual/08-misc instrs.gb",
    );
}

#[test]
#[ignore]
fn test_cpu_09() {
    blargg_test_common(
        "09",
        "../roms/gb-test-roms/cpu_instrs/individual/09-op r,r.gb",
    );
}

#[test]
#[ignore]
fn test_cpu_10() {
    blargg_test_common(
        "10",
        "../roms/gb-test-roms/cpu_instrs/individual/10-bit ops.gb",
    );
}

#[test]
#[ignore]
fn test_cpu_11() {
    blargg_test_common(
        "11",
        "../roms/gb-test-roms/cpu_instrs/individual/11-op a,(hl).gb",
    );
}
