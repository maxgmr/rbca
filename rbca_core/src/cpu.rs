//! All functionality related to the emulated CPU of the Game Boy.
use std::default::Default;

use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    instructions::execute_opcode,
    Button, Mmu, RegFlag, Registers,
    Target::{A, B, C, D, E, H, L},
    DISPLAY_HEIGHT, DISPLAY_WIDTH,
};

const INTERRUPT_FLAG_REGISTER_ADDR: u16 = 0xFF0F;
const INTERRUPT_ENABLE_REGISTER_ADDR: u16 = 0xFFFF;

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

    /// Perform one cycle. Return number of T-cycles taken.
    pub fn cycle(&mut self, debug: bool) -> u32 {
        self.update_interrupt_countdown();
        let interrupt = self.handle_interrupt();

        let t_cycles: u32 = if interrupt != 0 {
            interrupt
        } else if self.is_halted {
            4
        } else {
            let opcode = self.mmu.read_byte(self.pc);
            execute_opcode(self, opcode, debug)
        };

        self.mmu.cycle(t_cycles)
    }

    // Handle interrupts
    fn handle_interrupt(&mut self) -> u32 {
        if !self.interrupts_enabled && !self.is_halted {
            return 0;
        }

        let interrupt_enable_register = self.mmu.read_byte(INTERRUPT_ENABLE_REGISTER_ADDR);
        let interrupt_flag_register = self.mmu.read_byte(INTERRUPT_FLAG_REGISTER_ADDR);
        let interrupt_activated = interrupt_enable_register & interrupt_flag_register;
        if interrupt_activated == 0 {
            return 0;
        }

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
        self.mmu.write_byte(
            INTERRUPT_FLAG_REGISTER_ADDR,
            interrupt_flag_register & !(0b1 << offset),
        );
        self.push_stack(self.pc);
        self.pc = 0x0040 | ((offset as u16) << 3);
        16
    }

    fn update_interrupt_countdown(&mut self) {
        self.di_countdown = match self.di_countdown {
            2 => 1,
            1 => {
                self.interrupts_enabled = false;
                0
            }
            _ => 0,
        };
        self.ei_countdown = match self.ei_countdown {
            2 => 1,
            1 => {
                self.interrupts_enabled = true;
                0
            }
            _ => 0,
        }
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
