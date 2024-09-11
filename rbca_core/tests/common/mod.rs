use std::{
    fs::{self, OpenOptions},
    io::prelude::*,
    time::Instant,
};

use rbca_core::{
    Cpu, RegFlag, Registers,
    Target::{A, B, C, D, E, H, L},
};
use text_io::read;

pub fn blargg_test_common(rom_name: &str, rom_path: &str) {
    const INSTR_DEBUG: bool = true;
    // const INSTR_DEBUG: bool = false;
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

    let mut cpu = Cpu::new_cart(rom_path);

    println!("{}", cpu.mmu.cart.header_info());
    println!(
        "Instruction Debug: {}, Breakpoints: {} Slow: {} Logs: {}",
        INSTR_DEBUG,
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
        let cycles = cpu.cycle(INSTR_DEBUG);
        t_cycles += cycles;
        total_cycles += cycles as u128;

        if INSTR_DEBUG {
            println!("{blargg_out}");
        }

        // blargg output
        if cpu.mmu.read_byte(0xFF02) == 0x81 {
            let c: char = cpu.mmu.read_byte(0xFF01).into();
            blargg_out.push(c);
            if !INSTR_DEBUG {
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

#[derive(Debug, Clone)]
pub struct CpuState {
    pub pc: u16,
    pub sp: u16,
    pub regs: Registers,
    pub is_halted: bool,
    pub is_stopped: bool,
    pub di_countdown: usize,
    pub ei_countdown: usize,
    pub interrupts_enabled: bool,
}
impl CpuState {
    pub fn save_state(cpu: &Cpu) -> Self {
        Self {
            pc: cpu.pc,
            sp: cpu.sp,
            regs: cpu.regs.clone(),
            is_halted: cpu.is_halted,
            is_stopped: cpu.is_stopped,
            di_countdown: cpu.di_countdown,
            ei_countdown: cpu.ei_countdown,
            interrupts_enabled: cpu.interrupts_enabled,
        }
    }

    pub fn update(&mut self, cpu: &Cpu) {
        self.pc = cpu.pc;
        self.sp = cpu.sp;
        if self.regs != cpu.regs {
            self.regs = cpu.regs.clone()
        }
        self.is_halted = cpu.is_halted;
        self.is_stopped = cpu.is_stopped;
        self.di_countdown = cpu.di_countdown;
        self.ei_countdown = cpu.ei_countdown;
        self.interrupts_enabled = cpu.interrupts_enabled;
    }
}
