//! Functionality related to emulator memory.
use std::default::Default;

use crate::{io_registers::IORegisters, Flags};

/// Memory bus of the Game Boy.
#[derive(Debug, Clone)]
pub struct MemoryBus {
    /// [0x0000-0x3FFF] Cartridge ROM bank 0. Usually a fixed bank.
    pub cart_rom_0: [u8; 0x4000],
    /// [0x4000-0x7FFF] Switchable cartridge ROM bank.
    pub cart_rom_n: [u8; 0x4000],
    /// [0x8000-0x9FFF] Video RAM. Background & sprite data.
    pub vram: [u8; 0x2000],
    /// [0xA000-0xBFFF] External cartridge RAM. Programs that require more RAM than available can
    /// make more RAM addressable here.
    pub eram: [u8; 0x2000],
    /// [0xC000-0xDFFF] Work RAM. Read from or written to by CPU.
    pub wram: [u8; 0x2000],
    /// [0xE000-0xFDFF] Work RAM echo. Mirror of 0xC000-DDFF.
    pub wram_echo: [u8; 0x1E00],
    /// [0xFE00-0xFE9F] Object attribute memory. Stores data about rendered sprites (e.g.,
    /// positions, attributes, etc.).
    pub oam: [u8; 0x00A0],
    /// [0xFF00-0xFF7F] I/O registers. Allow programs to use the hardware subsystems (e.g.
    /// graphics, sound, etc.).
    pub io_regs: IORegisters,
    /// [0xFF80-0xFFFE] High RAM. High-speed RAM where the most interaction between the hardware
    /// and the program occurs.
    pub hram: [u8; 0x0080],
    /// [0xFFFF] Interrupt enable register.
    pub ie_reg: Flags,
}
impl MemoryBus {
    /// Create a new [MemoryBus].
    pub fn new() -> Self {
        Self {
            cart_rom_0: [0x00; 0x4000],
            cart_rom_n: [0x00; 0x4000],
            vram: [0x00; 0x2000],
            eram: [0x00; 0x2000],
            wram: [0x00; 0x2000],
            wram_echo: [0x00; 0x1E00],
            oam: [0x00; 0x00A0],
            io_regs: IORegisters::new(),
            hram: [0x00; 0x0080],
            ie_reg: Flags::new(0b0000_0000),
        }
    }

    /// Read the byte at the given address.
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.cart_rom_0[address as usize],
            0x4000..=0x7FFF => self.cart_rom_n[address as usize - 0x4000],
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xA000..=0xBFFF => self.eram[address as usize - 0xA000],
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000],
            0xE000..=0xFDFF => self.eram[address as usize - 0xE000],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            // TODO should probably not just instantly panic- maybe warning?
            0xFEA0..=0xFEFF => {
                panic!("Attempted to read unusable memory address {:#04X}", address)
            }
            0xFF00..=0xFF7F => self.io_regs.read_byte(address - 0xFF00),
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
            0xFFFF => self.ie_reg.read_byte(),
        }
    }

    /// Read 2 bytes from the given start address. (little-endian)
    pub fn read_2_bytes(&mut self, start_address: u16) -> u16 {
        (self.read_byte(start_address) as u16) | ((self.read_byte(start_address + 1) as u16) << 8)
    }

    /// Write a byte to a given address.
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            0x0000..=0x3FFF => self.cart_rom_0[address as usize] = byte,
            0x4000..=0x7FFF => self.cart_rom_n[address as usize - 0x4000] = byte,
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = byte,
            0xA000..=0xBFFF => self.eram[address as usize - 0xA000] = byte,
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000] = byte,
            0xE000..=0xFDFF => self.eram[address as usize - 0xE000] = byte,
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = byte,
            // TODO should probably not just instantly panic- maybe warning?
            0xFEA0..=0xFEFF => panic!(
                "Attempted to write to unusable memory address {:#04X}",
                address
            ),
            0xFF00..=0xFF7F => self.io_regs.write_byte(address - 0xFF00, byte),
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = byte,
            0xFFFF => self.ie_reg.write_byte(byte),
        };
    }

    /// Write 2 bytes to a given start address. (little-endian)
    pub fn write_2_bytes(&mut self, start_address: u16, value: u16) {
        self.write_byte(start_address, (value & 0x00FF) as u8);
        self.write_byte(start_address + 1, (value >> 8) as u8);
    }
}
impl Default for MemoryBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_read_write() {
        let mut mem_bus = MemoryBus::new();
        mem_bus.write_byte(0x0000, 0x12);
        mem_bus.write_2_bytes(0x0001, 0x3456);
        assert_eq!(mem_bus.read_byte(0x0000), 0x12);
        assert_eq!(mem_bus.read_byte(0x0001), 0x56);
        assert_eq!(mem_bus.read_byte(0x0002), 0x34);
        assert_eq!(mem_bus.read_2_bytes(0x0000), 0x5612);
        assert_eq!(mem_bus.read_2_bytes(0x0001), 0x3456);
        assert_eq!(mem_bus.read_byte(0x0002), 0x34);
        mem_bus.write_byte(0x0001, 0xFB);
        assert_eq!(mem_bus.read_byte(0x0000), 0x12);
        assert_eq!(mem_bus.read_byte(0x0001), 0xFB);
        assert_eq!(mem_bus.read_byte(0x0002), 0x34);
    }
}
