use std::default::Default;

use crate::{Flags, FlagsEnum, PPU};

const DISABLE_BOOT_ROM: bool = true;

/// I/O registers of memory.
#[derive(Debug, Clone)]
pub struct IORegisters {
    joypad: Flags,
    st_data: u8,
    st_control: Flags,
    // Helper counter
    timer_div_clocksum: u32,
    // Helper counter
    timer_clocksum: u32,
    divider: u8,
    timer_counter: u8,
    timer_modulo: u8,
    timer_control: Flags,
    /// IF (interrupt flags) register.
    pub interrupt_flags: Flags,
    // TODO audio
    audio: [u8; 0x11],
    // TODO wave pattern
    wave_pattern: [u8; 0x10],
    /// Picture processing unit.
    pub ppu: PPU,
    disable_boot_rom: u8,
    // TODO CGB only
    vram_dma: [u8; 0x05],
    // TODO CGB only
    bg_obj_palettes: [u8; 0x04],
    // TODO CGB only
    wram_bank_select: u8,
}
impl IORegisters {
    /// Create new [IORegisters].
    pub fn new() -> Self {
        Self {
            // All buttons released
            joypad: Flags::new(0b0011_1111),
            st_data: 0x00,
            st_control: Flags::new(0b0000_0000),
            timer_div_clocksum: 0x0000_0000,
            timer_clocksum: 0x0000_0000,
            divider: 0x00,
            timer_counter: 0x00,
            timer_modulo: 0x00,
            // Don't increment TIMA. TIMA increment speed = every 256 M-cycles.
            timer_control: Flags::new(0b0000_0000),
            // No interrupt flags requested by default.
            interrupt_flags: Flags::new(0b0000_0000),
            // TODO audio
            audio: [0x00; 0x11],
            // TODO wave pattern
            wave_pattern: [0x00; 0x10],
            // TODO graphics
            ppu: PPU::new(),
            disable_boot_rom: if DISABLE_BOOT_ROM { 1 } else { 0 },
            // TODO
            vram_dma: [0x00; 0x05],
            // TODO
            bg_obj_palettes: [0x00; 0x04],
            // TODO
            wram_bank_select: 0x00,
        }
    }

    /// Directly retrieve the byte at the given address. Not recommended.
    pub fn read_byte(&self, address: u16) -> u8 {
        // Initial offset: 0xFF00
        match address {
            0x0000 => self.joypad.read_byte(),
            0x0001 => self.st_data,
            0x0002 => self.st_control.read_byte(),
            0x0004 => self.divider,
            0x0005 => self.timer_counter,
            0x0006 => self.timer_modulo,
            0x0007 => self.timer_control.read_byte(),
            0x000F => self.interrupt_flags.read_byte(),
            0x0010..=0x0026 => self.audio[address as usize - 0x0010],
            0x0030..=0x003F => self.wave_pattern[address as usize - 0x0030],
            0x0040..=0x004F => self.ppu.read_byte(address - 0x0040),
            0x0050 => self.disable_boot_rom,
            0x0051..=0x0055 => self.vram_dma[address as usize - 0x0051],
            0x0068..=0x006B => self.bg_obj_palettes[address as usize - 0x0068],
            0x0070 => self.wram_bank_select,
            _ => 0xFF,
        }
    }

    /// Directly replace the byte at the given address. Not recommended.
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        // Initial offset: 0xFF00
        match address {
            0x0000 => self.joypad.write_byte(byte),
            0x0001 => self.st_data = byte,
            0x0002 => self.st_control.write_byte(byte),
            0x0004 => self.divider = byte,
            0x0005 => self.timer_counter = byte,
            0x0006 => self.timer_modulo = byte,
            0x0007 => self.timer_control.write_byte(byte),
            0x000F => self.interrupt_flags.write_byte(byte),
            0x0010..=0x0026 => self.audio[address as usize - 0x0010] = byte,
            0x0030..=0x003F => self.wave_pattern[address as usize - 0x0030] = byte,
            0x0040..=0x004F => self.ppu.write_byte(address - 0x0040, byte),
            0x0050 => self.disable_boot_rom = byte,
            0x0051..=0x0055 => self.vram_dma[address as usize - 0x0051] = byte,
            0x0068..=0x006B => self.bg_obj_palettes[address as usize - 0x0068] = byte,
            0x0070 => self.wram_bank_select = byte,
            _ => {}
        }
    }

    /// Perform one timer cycle.
    pub fn timer_cycle(&mut self, t_cycles: u32) {
        // Set timer divider.
        self.timer_div_clocksum += t_cycles;
        if self.timer_div_clocksum >= 256 {
            self.divider = self.divider.wrapping_add(1);
            self.timer_div_clocksum -= 256;
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
impl Default for IORegisters {
    fn default() -> Self {
        Self::new()
    }
}

/// Joypad enum.
#[derive(Debug, Copy, Clone)]
enum Joyp {
    /// If 0, buttons (SsBA) can be read from lower nibble.
    SelectButtons,
    /// If 0, directional keys can be read from lower nibble.
    SelectDPad,
    /// 0 = pressed.
    StartDown,
    /// 0 = pressed.
    SelectUp,
    /// 0 = pressed.
    BLeft,
    /// 0 = pressed.
    ARight,
}
impl FlagsEnum for Joyp {
    fn val(&self) -> u8 {
        match self {
            Self::SelectButtons => 0b0010_0000,
            Self::SelectDPad => 0b0001_0000,
            Self::StartDown => 0b0000_1000,
            Self::SelectUp => 0b0000_0100,
            Self::BLeft => 0b0000_0010,
            Self::ARight => 0b0000_0001,
        }
    }
}

/// Serial transfer control enum.
#[derive(Debug, Copy, Clone)]
enum Stc {
    /// If 1, transfer is either requested or in progress.
    TransferEnable,
    /// (CGB mode only) If 1, enable high speed serial clock (~256 kHz in single-speed mode)
    ClockSpeed,
    /// If 0, External/slave clock, 1 = Internal/master clock.
    ClockSelect,
}
impl FlagsEnum for Stc {
    fn val(&self) -> u8 {
        match self {
            Self::TransferEnable => 0b1000_0000,
            Self::ClockSpeed => 0b0000_0010,
            Self::ClockSelect => 0b0000_0001,
        }
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

/// Interrupt flags enum. Controls whether the different interrupt handlers are being requested.
#[derive(Debug, Copy, Clone)]
pub enum If {
    /// Joypad interrupt handler.
    Joypad,
    /// Serial interrupt handler.
    Serial,
    /// Timer interrupt handler.
    Timer,
    /// LCD interrupt handler.
    Lcd,
    /// VBlank interrupt handler.
    VBlank,
}
impl FlagsEnum for If {
    fn val(&self) -> u8 {
        match self {
            Self::Joypad => 0b0001_0000,
            Self::Serial => 0b0000_1000,
            Self::Timer => 0b0000_0100,
            Self::Lcd => 0b0000_0010,
            Self::VBlank => 0b0000_0001,
        }
    }
}
