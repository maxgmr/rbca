use std::default::Default;

use crate::{mmu::If, Flags, FlagsEnum};

/// Device timer.
#[derive(Debug)]
pub struct Timer {
    // Used to alert MMU that the timer triggered some interrupt flags.
    pub interrupt_flags: Flags,
    // Internal helper counter.
    timer_internal_clocksum: u32,
    timer_clocksum: u32,
    divider: u8,
    timer_counter: u8,
    timer_modulo: u8,
    timer_control: Flags,
}
impl Timer {
    /// Create a new [Timer].
    pub fn new() -> Self {
        Self {
            interrupt_flags: Flags::new(0b0000_0000),
            timer_internal_clocksum: 0,
            timer_clocksum: 0,
            divider: 0x00,
            timer_counter: 0x00,
            timer_modulo: 0x00,
            timer_control: Flags::new(0b0000_0000),
        }
    }

    /// Directly read the byte at the given address.
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.divider,
            0xFF05 => self.timer_counter,
            0xFF06 => self.timer_modulo,
            0xFF07 => self.timer_control.read_byte(),
            _ => panic!("Timer: read illegal address {:#06X}.", address),
        }
    }

    /// Directly write to the byte at the given address.
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.divider = 0x00,
            0xFF05 => self.timer_counter = value,
            0xFF06 => self.timer_modulo = value,
            0xFF07 => self.timer_control.write_byte(value),
            _ => panic!("Timer: write illegal address {:#06X}.", address),
        }
    }

    /// Perform one timer cycle.
    pub fn cycle(&mut self, t_cycles: u32) {
        // Set timer divider.
        self.timer_internal_clocksum += t_cycles;
        if self.timer_internal_clocksum >= 256 {
            self.divider = self.divider.wrapping_add(1);
            self.timer_internal_clocksum -= 256;
        }

        // Check whether TIMA gets incremented.
        if self.timer_control.get(Tac::Enable) {
            // Increase helper counter.
            self.timer_clocksum += t_cycles;

            // Get frequency from TAC register.
            // Increment every <frequency> t-cycles.
            let frequency = match (
                self.timer_control.get(Tac::ClockSelect1),
                self.timer_control.get(Tac::ClockSelect0),
            ) {
                // Increment every...
                // 0b11: 64 m-cycles
                (true, true) => 64 * 4,
                // 0b01: 4 m-cycles
                (false, true) => 4 * 4,
                // 0b10: 16 m-cycles
                (true, false) => 16 * 4,
                // 0b00: 256 m-cycles
                _ => 256 * 4,
            };

            // Increment the timer according to the frequency.
            while self.timer_clocksum >= frequency {
                // Increase TIMA
                self.timer_counter = self.timer_counter.wrapping_add(1);

                // If TIMA overflows...
                if self.timer_counter == 0x00 {
                    // eset to TMA value.
                    self.timer_counter = self.timer_modulo;
                    // Request an interrupt.
                    self.interrupt_flags.set(If::Timer, true);
                }
            }
        }
    }
}
impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer control enum.
#[derive(Debug, Copy, Clone)]
enum Tac {
    /// If 1, increment TIMA.
    Enable,
    /// Bit 1 of clock select.
    ClockSelect1,
    /// Bit 0 of clock select.
    ClockSelect0,
}
impl FlagsEnum for Tac {
    fn val(&self) -> u8 {
        match self {
            Self::Enable => 0b0000_0100,
            Self::ClockSelect1 => 0b0000_0010,
            Self::ClockSelect0 => 0b0000_0001,
        }
    }
}
