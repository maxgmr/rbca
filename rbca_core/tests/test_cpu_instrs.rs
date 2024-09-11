#![cfg(test)]

use std::{
    fs::{self, OpenOptions},
    io::prelude::*,
    time::Instant,
};

mod common;

use common::CpuState;

use rbca_core::{
    Cpu, RegFlag,
    Target::{A, B, C, D, E, H, L},
    DEBUG_INSTRUCTIONS,
};
use text_io::read;

fn test_common(rom_name: &str) {
    // const BREAKPOINTS: bool = true;
    const BREAKPOINTS: bool = false;
    // const SLOW: bool = true;
    const SLOW: bool = false;
    const WAIT_MS: u64 = 50;
    // const LOG: bool = true;
    const LOG: bool = false;

    #[allow(unused_variables)]
    fn is_breakpoint(
        cpu: &Cpu,
        last_state: &CpuState,
        total_cycles: u128,
        total_steps: u128,
    ) -> bool {
        total_steps == 2496
        // cpu.pc == 0xDEF8
        // cpu.regs.get_reg(D) != last_state.regs.get_reg(D)
        // (cpu.regs.get_flag(RegFlag::C) != last_state.regs.get_flag(RegFlag::C)) || (cpu.pc > 0xC000)
    }

    // #[allow(unused_assignments)]
    // let mut log_file = OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .truncate(LOG)
    //     .open(format!("{}_LOG", &rom_name[0..=1]))
    //     .unwrap();

    let file_name = format!("{}_LOG", &rom_name[0..=1]);
    let _ = fs::remove_file(&file_name);
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .unwrap();

    let mut cpu = Cpu::new_cart(format!(
        "../roms/gb-test-roms/cpu_instrs/individual/{}",
        rom_name
    ));

    println!("{}", cpu.mmu.cart.header_info());
    println!(
        "Breakpoints: {} Slow: {} Logs: {}",
        BREAKPOINTS,
        if SLOW {
            format!("{} ms", WAIT_MS)
        } else {
            "false".to_owned()
        },
        LOG
    );
    println!("Enter any text to continue...");
    let _: String = read!();
    println!("-------");

    cpu.pc = 0x0100;
    cpu.sp = 0xFFFE;
    cpu.mmu
        .write_byte(0xFF40, cpu.mmu.read_byte(0xFF40) | 0b1000_0000);
    cpu.regs.set_reg(A, 0x01);
    cpu.regs.set_reg(B, 0x00);
    cpu.regs.set_reg(C, 0x13);
    cpu.regs.set_reg(D, 0x00);
    cpu.regs.set_reg(E, 0xD8);
    cpu.regs.set_reg(H, 0x01);
    cpu.regs.set_reg(L, 0x4D);
    cpu.regs.set_flag(RegFlag::Z, true);
    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, true);
    cpu.regs.set_flag(RegFlag::C, true);

    let mut last_state = CpuState::save_state(&cpu);
    let mut last_break: bool = false;
    let mut last_step_forward: bool = false;

    let mut t_cycles = 0;
    let mut total_cycles: u128 = 0;
    let mut total_steps: u128 = 0;

    let mut blargg_out = String::new();

    let mut log_queue: String = String::with_capacity(18000 * 200);

    loop {
        total_steps += 1;
        // log file
        // gameboy-doctor version
        if LOG {
            log_queue.push_str(&format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n", cpu.regs.get_reg(A), cpu.regs.get_flag_byte(), cpu.regs.get_reg(B), cpu.regs.get_reg(C), cpu.regs.get_reg(D), cpu.regs.get_reg(E), cpu.regs.get_reg(H), cpu.regs.get_reg(L), cpu.sp, cpu.pc, cpu.mmu.read_byte(cpu.pc), cpu.mmu.read_byte(cpu.pc + 1), cpu.mmu.read_byte(cpu.pc + 2), cpu.mmu.read_byte(cpu.pc + 3)));
        }
        // logdbg version
        // if LOG {
        //     log_queue.push_str(&format!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})\n", cpu.regs.get_reg(A), cpu.regs.get_flag_byte(), cpu.regs.get_reg(B), cpu.regs.get_reg(C), cpu.regs.get_reg(D), cpu.regs.get_reg(E), cpu.regs.get_reg(H), cpu.regs.get_reg(L), cpu.sp, cpu.pc, cpu.mem_bus.read_byte(cpu.pc), cpu.mem_bus.read_byte(cpu.pc + 1), cpu.mem_bus.read_byte(cpu.pc + 2), cpu.mem_bus.read_byte(cpu.pc + 3)));
        // }

        last_state.update(&cpu);
        let t_start = Instant::now();
        let cycles = cpu.cycle();
        t_cycles += cycles;
        total_cycles += cycles as u128;

        if DEBUG_INSTRUCTIONS {
            println!("{blargg_out}");
        }

        // blargg output
        if cpu.mmu.read_byte(0xFF02) == 0x81 {
            let c: char = cpu.mmu.read_byte(0xFF01).into();
            blargg_out.push(c);
            if !DEBUG_INSTRUCTIONS {
                print!("{c}");
            }
            cpu.mmu.write_byte(0xFF02, 0x00);
        }

        // breakpoints
        if BREAKPOINTS && last_break {
            last_break = false;
            last_step_forward = true;
            println!(" - BREAK @ {total_steps} - ");
            let _: String = read!();
        } else if BREAKPOINTS && last_step_forward {
            last_step_forward = false;
            println!(" - STEP FORWARD - ");
            let _: String = read!();
        }
        if BREAKPOINTS && is_breakpoint(&cpu, &last_state, total_cycles, total_steps + 1) {
            last_break = true;
        }

        if SLOW {
            std::thread::sleep(std::time::Duration::from_millis(WAIT_MS));
        }

        if t_cycles >= 17476 {
            t_cycles %= 17476;
            while t_start.elapsed().as_millis() < 16 {}
            if LOG {
                if let Err(e) = write!(log_file, "{log_queue}") {
                    eprintln!("ERROR: {}", e);
                }
                log_queue = String::new();
            }
        }
    }
}

// Example usage: cargo t 01 -- --nocapture --ignored
#[test]
#[ignore]
fn test_cpu_01() {
    test_common("01-special.gb");
}

#[test]
#[ignore]
fn test_cpu_02() {
    test_common("02-interrupts.gb");
}

#[test]
#[ignore]
fn test_cpu_03() {
    test_common("03-op sp,hl.gb");
}

#[test]
#[ignore]
fn test_cpu_04() {
    test_common("04-op r,imm.gb");
}

#[test]
#[ignore]
fn test_cpu_05() {
    test_common("05-op rp.gb");
}

#[test]
#[ignore]
fn test_cpu_06() {
    test_common("06-ld r,r.gb");
}

#[test]
#[ignore]
fn test_cpu_07() {
    test_common("07-jr,jp,call,ret,rst.gb");
}

#[test]
#[ignore]
fn test_cpu_08() {
    test_common("08-misc instrs.gb");
}

#[test]
#[ignore]
fn test_cpu_09() {
    test_common("09-op r,r.gb");
}

#[test]
#[ignore]
fn test_cpu_10() {
    test_common("10-bit ops.gb");
}

#[test]
#[ignore]
fn test_cpu_11() {
    test_common("11-op a,(hl).gb");
}
