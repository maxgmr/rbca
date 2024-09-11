#![cfg(test)]

mod common;

use common::blargg_test_common;

#[test]
#[ignore]
fn test_mem1_all() {
    blargg_test_common(
        "MEM_TIME_1_ALL",
        "../roms/gb-test-roms/mem_timing/mem_timing.gb",
    );
}

#[test]
#[ignore]
fn test_mem1_01() {
    blargg_test_common(
        "MEM_TIME_1_01",
        "../roms/gb-test-roms/mem_timing/individual/01-read_timing.gb",
    );
}

#[test]
#[ignore]
fn test_mem1_02() {
    blargg_test_common(
        "MEM_TIME_1_02",
        "../roms/gb-test-roms/mem_timing/individual/02-write_timing.gb",
    );
}

#[test]
#[ignore]
fn test_mem1_03() {
    blargg_test_common(
        "MEM_TIME_1_03",
        "../roms/gb-test-roms/mem_timing/individual/03-modify_timing.gb",
    );
}

#[test]
#[ignore]
fn test_mem2_all() {
    blargg_test_common(
        "MEM_TIME_1_ALL",
        "../roms/gb-test-roms/mem_timing-2/mem_timing.gb",
    );
}

#[test]
#[ignore]
fn test_mem2_01() {
    blargg_test_common(
        "MEM_TIME_1_01",
        "../roms/gb-test-roms/mem_timing-2/rom_singles/01-read_timing.gb",
    );
}

#[test]
#[ignore]
fn test_mem2_02() {
    blargg_test_common(
        "MEM_TIME_1_02",
        "../roms/gb-test-roms/mem_timing-2/rom_singles/02-write_timing.gb",
    );
}

#[test]
#[ignore]
fn test_mem2_03() {
    blargg_test_common(
        "MEM_TIME_1_03",
        "../roms/gb-test-roms/mem_timing-2/rom_singles/03-modify_timing.gb",
    );
}
