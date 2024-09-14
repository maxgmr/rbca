//! All functionality related to the emulated CPU of the Game Boy.
use std::{default::Default, fmt::Display};

use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    instructions::execute_opcode,
    Button, Mmu, RegFlag, Registers,
    Target::{A, B, C, D, E, H, L},
    DISPLAY_HEIGHT, DISPLAY_WIDTH,
};

/// The emulated CPU of the Game Boy.
#[derive(Debug)]
pub struct Cpu {
    /// Registers
    pub regs: Registers,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u16,
    /// MMU
    pub mmu: Mmu,
    /// Halted
    pub is_halted: bool,
    /// Stopped
    // TODO
    pub is_stopped: bool,
    /// Countdown until interrupts are disabled.
    pub di_countdown: usize,
    /// Countdown until interrupts are enabled.
    pub ei_countdown: usize,
    /// Whether interrupts are enabled or not.
    pub interrupts_enabled: bool,
}
impl Cpu {
    /// Create a new [Cpu] with no boot ROM or cartridge.
    pub fn new() -> Self {
        Self::new_no_boot_helper(None::<Utf8PathBuf>)
    }

    /// Create a new [Cpu] with no boot ROM.
    pub fn new_cart<P: AsRef<Utf8Path>>(cart_path: P) -> Self {
        Self::new_no_boot_helper(Some(cart_path))
    }

    /// Create a new [Cpu] with no cartridge.
    pub fn new_boot<P: AsRef<Utf8Path>>(boot_rom_path: P) -> Self {
        Self::new_with_boot_helper(None, boot_rom_path)
    }

    /// Create a new [Cpu] with a cartridge and a boot ROM.
    pub fn new_boot_cart<P: AsRef<Utf8Path>>(cart_path: P, boot_rom_path: P) -> Self {
        Self::new_with_boot_helper(Some(cart_path), boot_rom_path)
    }

    fn new_with_boot_helper<P: AsRef<Utf8Path>>(cart_path: Option<P>, boot_rom_path: P) -> Self {
        // TODO random vals
        Self {
            regs: Registers::new(),
            pc: 0x0000,
            sp: 0x0000,
            mmu: if let Some(cp) = cart_path {
                Mmu::new_boot_cart(cp, boot_rom_path)
            } else {
                Mmu::new_boot(boot_rom_path)
            },
            is_halted: false,
            is_stopped: false,
            ei_countdown: 0,
            di_countdown: 0,
            interrupts_enabled: false,
        }
    }

    fn new_no_boot_helper<P: AsRef<Utf8Path>>(cart_path: Option<P>) -> Self {
        let mut cpu = Self {
            regs: Registers::new_after_boot_rom(),
            pc: 0x0100,
            sp: 0xFFFE,
            mmu: if let Some(cp) = cart_path {
                Mmu::new_cart(cp)
            } else {
                Mmu::new()
            },
            is_halted: false,
            is_stopped: false,
            ei_countdown: 0,
            di_countdown: 0,
            interrupts_enabled: false,
        };
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
        cpu
    }

    /// Get the output of the PPU. This is an array of values ranging from 0-3. Each value
    /// represents one pixel.
    ///
    /// To render this output, perform the following conversion:
    /// 0 -> White
    /// 1 -> Light grey
    /// 2 -> Dark grey
    /// 3 -> Black
    pub fn get_pixels(&self) -> &[u8; DISPLAY_WIDTH * DISPLAY_HEIGHT] {
        &self.mmu.ppu.data_output
    }

    /// Handle a button press.
    pub fn button_down(&mut self, button: Button, debug: bool) {
        let data = self.mmu.joypad.button_down(button);
        if debug {
            println!("BUTTON_DOWN: {button} DATA: {:010b}", data.read_byte());
        }
    }

    /// Handle a button release.
    pub fn button_up(&mut self, button: Button, debug: bool) {
        let data = self.mmu.joypad.button_up(button);
        if debug {
            println!("BUTTON_UP: {button} DATA: {:010b}", data.read_byte());
        }
    }

    /// Perform one cycle. Return number of T-cycles taken and any debug info.
    pub fn cycle(&mut self, debug: bool, get_state: bool) -> (u32, Option<EmuState>) {
        self.update_interrupt_countdown();
        let interrupt = self.handle_interrupt();
        let cycles_and_state = if interrupt != 0 {
            let mut emu_state = EmuState::new(self);
            emu_state.update(0, interrupt, "INTERRUPT".to_owned());
            (interrupt, Some(emu_state))
        } else if self.is_halted {
            (4, None)
        } else {
            let opcode = self.mmu.read_byte(self.pc);
            execute_opcode(self, opcode, debug, get_state)
        };

        self.mmu.cycle(cycles_and_state.0);
        (cycles_and_state.0, cycles_and_state.1)
    }

    // Handle interrupts
    fn handle_interrupt(&mut self) -> u32 {
        if !self.interrupts_enabled && !self.is_halted {
            return 0;
        }

        let interrupt_enable_register = 0b0001_1111 & self.mmu.read_byte(0xFFFF);
        let interrupt_flag_register = 0b0001_1111 & self.mmu.read_byte(0xFF0F);
        let interrupt_activated = interrupt_enable_register & interrupt_flag_register;
        if interrupt_activated == 0 {
            return 0;
        }

        let halted_penalty = if self.is_halted { 4 } else { 0 };

        self.is_halted = false;
        if !self.interrupts_enabled {
            return 0;
        }
        self.interrupts_enabled = false;

        // Prioritise lowest activated interrupt
        let offset = interrupt_activated.trailing_zeros();
        if offset >= 5 {
            panic!("Invalid interrupt triggered: {:#010b}", interrupt_activated);
        }
        self.mmu
            .write_byte(0xFF0F, interrupt_flag_register & !(0b1 << offset));
        self.push_stack(self.pc);
        self.pc = 0x0040 | ((offset as u16) << 3);
        20 + halted_penalty
    }

    fn update_interrupt_countdown(&mut self) {
        self.ei_countdown = match self.ei_countdown {
            2 => 1,
            1 => {
                self.interrupts_enabled = true;
                0
            }
            _ => 0,
        };
        self.di_countdown = match self.di_countdown {
            2 => 1,
            1 => {
                self.interrupts_enabled = false;
                0
            }
            _ => 0,
        };
    }

    /// Load something into memory.
    #[cfg(test)]
    pub fn load(&mut self, start_index: u16, data: &[u8]) {
        for (i, _) in data.iter().enumerate() {
            self.mmu.write_byte(start_index + (i as u16), data[i]);
        }
    }

    /// Get next byte.
    pub fn get_next_byte(&mut self) -> u8 {
        self.mmu.read_byte(self.pc + 1)
    }

    /// Get next two bytes (little-endian).
    pub fn get_next_2_bytes(&mut self) -> u16 {
        self.mmu.read_2_bytes(self.pc + 1)
    }

    /// Push to stack.
    pub fn push_stack(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.mmu.write_2_bytes(self.sp, value);
    }

    /// Pop from stack.
    pub fn pop_stack(&mut self) -> u16 {
        let popped_val = self.mmu.read_2_bytes(self.sp);
        self.sp = self.sp.wrapping_add(2);
        popped_val
    }

    /// Get some emulator info formatted as a nice String.
    pub fn emu_info(&self) -> String {
        format!(
            "RBCA Emulator Info
\tBoot ROM\t\t{}

{}",
            if self.mmu.boot_rom.is_some() {
                "Yes"
            } else {
                "No"
            },
            if self.mmu.cart.is_empty() {
                String::from("No Cartridge")
            } else {
                self.mmu.cart.header_info()
            }
        )
    }
}
impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

/// The state of the emulator at a specific point in time.
pub struct EmuState {
    /// Program counter.
    pub pc: u16,
    /// Stack pointer.
    pub sp: u16,
    /// Byte at the program counter.
    pub byte_0: u8,
    /// Byte after the program counter.
    pub byte_1: u8,
    /// Byte 2 after the program counter.
    pub byte_2: u8,
    /// Zero flag.
    pub z_flag: bool,
    /// Subtraction flag.
    pub n_flag: bool,
    /// Half-carry flag.
    pub h_flag: bool,
    /// Carry flag.
    pub c_flag: bool,
    /// A register.
    pub a_reg: u8,
    /// B register.
    pub b_reg: u8,
    /// C register.
    pub c_reg: u8,
    /// D register.
    pub d_reg: u8,
    /// E register.
    pub e_reg: u8,
    /// H register.
    pub h_reg: u8,
    /// L register.
    pub l_reg: u8,
    /// Instruction size.
    pub size: u16,
    /// Instruction cycles.
    pub cycles: u32,
    /// Instruction string.
    pub instruction_string: String,
}
impl EmuState {
    /// Create an EmuState from the current state of the CPU. Default fields for instruction info.
    pub fn new(cpu: &Cpu) -> Self {
        Self {
            pc: cpu.pc,
            sp: cpu.sp,
            byte_0: cpu.mmu.read_byte(cpu.pc),
            byte_1: cpu.mmu.read_byte(cpu.pc + 1),
            byte_2: cpu.mmu.read_byte(cpu.pc + 2),
            z_flag: cpu.regs.get_flag(RegFlag::Z),
            n_flag: cpu.regs.get_flag(RegFlag::N),
            h_flag: cpu.regs.get_flag(RegFlag::H),
            c_flag: cpu.regs.get_flag(RegFlag::C),
            a_reg: cpu.regs.get_reg(A),
            b_reg: cpu.regs.get_reg(B),
            c_reg: cpu.regs.get_reg(C),
            d_reg: cpu.regs.get_reg(D),
            e_reg: cpu.regs.get_reg(E),
            h_reg: cpu.regs.get_reg(H),
            l_reg: cpu.regs.get_reg(L),
            size: 0,
            cycles: 0,
            instruction_string: String::new(),
        }
    }

    /// Populate the empty fields with the info from the executed instruction.
    pub fn update(&mut self, instr_size: u16, instr_cycles: u32, instr_string: String) {
        self.size = instr_size;
        self.cycles = instr_cycles;
        self.instruction_string = instr_string;
    }
}
impl Display for EmuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = format!("{:#04X}", self.byte_0);
        if self.size > 1 {
            data.push_str(&format!(" {:#04X}", self.byte_1));
        }
        if self.size > 2 {
            data.push_str(&format!(" {:#04X}", self.byte_2));
        }

        let flags: [char; 4] = [
            if self.z_flag { 'Z' } else { '-' },
            if self.n_flag { 'N' } else { '-' },
            if self.h_flag { 'H' } else { '-' },
            if self.c_flag { 'C' } else { '-' },
        ];
        write!(
            f,
            "{:#06X} | {:<14} | {:<10} | A: {:02X} F: {} BC: {:04X} DE: {:04X} HL: {:04X}",
            self.pc,
            data,
            self.instruction_string,
            self.a_reg,
            flags.iter().collect::<String>(),
            ((self.b_reg as u16) << 8) | (self.c_reg as u16),
            ((self.d_reg as u16) << 8) | (self.e_reg as u16),
            ((self.h_reg as u16) << 8) | (self.l_reg as u16),
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_next_2_bytes() {
        let mut cpu = Cpu::new();
        let data = [0x01, 0x23, 0x45, 0x67];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;
        assert_eq!(cpu.get_next_2_bytes(), 0x4523);
    }
}
