//! All functionality related to the emulated CPU of the Game Boy.
use std::default::Default;

use crate::{instructions::execute_opcode, MemoryBus, Registers};

const INTERRUPT_FLAG_REGISTER_ADDR: u16 = 0xFF0F;
const INTERRUPT_ENABLE_REGISTER_ADDR: u16 = 0xFFFF;

/// The emulated CPU of the Game Boy.
#[derive(Debug, Clone)]
pub struct Cpu {
    /// Registers
    pub regs: Registers,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u16,
    /// Memory
    pub mem_bus: MemoryBus,
    /// Halted
    pub is_halted: bool,
    /// Stopped
    pub is_stopped: bool,
    /// Countdown until interrupts are disabled.
    pub di_countdown: usize,
    /// Countdown until interrupts are enabled.
    pub ei_countdown: usize,
    /// Whether interrupts are enabled or not.
    pub interrupts_enabled: bool,
}
impl Cpu {
    /// Create a new [Cpu].
    pub fn new() -> Self {
        Self::default()
    }

    /// Perform one cycle. Return number of T-cycles taken.
    pub fn cycle(&mut self) -> u32 {
        self.update_interrupt_countdown();
        match self.handle_interrupt() {
            0 => {}
            ticks => return ticks,
        }

        if self.is_halted {
            4
        } else {
            let opcode = self.mem_bus.read_byte(self.pc);
            execute_opcode(self, opcode)
        }
    }

    // Handle interrupts
    fn handle_interrupt(&mut self) -> u32 {
        if !self.interrupts_enabled && !self.is_halted {
            return 0;
        }

        let interrupt_enable_register = self.mem_bus.read_byte(INTERRUPT_ENABLE_REGISTER_ADDR);
        let interrupt_flag_register = self.mem_bus.read_byte(INTERRUPT_FLAG_REGISTER_ADDR);
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
        self.mem_bus.write_byte(
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
            self.mem_bus.write_byte(start_index + (i as u16), data[i]);
        }
    }

    /// Get next byte.
    pub fn get_next_byte(&mut self) -> u8 {
        self.mem_bus.read_byte(self.pc + 1)
    }

    /// Get next two bytes (little-endian).
    pub fn get_next_2_bytes(&mut self) -> u16 {
        self.mem_bus.read_2_bytes(self.pc + 1)
    }

    /// Push to stack.
    pub fn push_stack(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.mem_bus.write_2_bytes(self.sp, value);
    }

    /// Pop from stack.
    pub fn pop_stack(&mut self) -> u16 {
        let popped_val = self.mem_bus.read_2_bytes(self.sp);
        self.sp += 2;
        popped_val
    }
}
impl Default for Cpu {
    fn default() -> Self {
        Self {
            regs: Registers::default(),
            pc: u16::default(),
            sp: 0xFFFE,
            mem_bus: MemoryBus::new(),
            is_halted: false,
            is_stopped: false,
            ei_countdown: 0,
            di_countdown: 0,
            interrupts_enabled: false,
        }
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
        assert_eq!(cpu.get_next_2_bytes(), 0x4523);
    }
}
