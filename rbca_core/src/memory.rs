//! Functionality related to emulator memory.
use std::default::Default;

use crate::{io_registers::IORegisters, Cartridge, Flags};

/// Memory bus of the Game Boy.
#[derive(Debug, Clone)]
pub struct MemoryBus {
    /// The [Cartridge] loaded from the computer.
    pub cart: Option<Cartridge>,
    /// [0x0000-0x3FFF] Cartridge ROM bank 0. Usually a fixed bank.
    pub cart_rom_0: [u8; 0x4000],
    /// [0x4000-0x7FFF] Switchable cartridge ROM bank 1.
    pub cart_rom_1: [u8; 0x4000],
    /// [0x4000-0x7FFF] Switchable cartridge ROM bank 2.
    pub cart_rom_2: [u8; 0x4000],
    /// [0x4000-0x7FFF] Switchable cartridge ROM bank 3.
    pub cart_rom_3: [u8; 0x4000],
    /// [0x4000-0x7FFF] Switchable cartridge ROM bank 5.
    pub cart_rom_5: [u8; 0x4000],
    /// Currently-selected cartridge ROM bank.
    current_rom_bank: usize,
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
    pub hram: [u8; 0x007F],
    /// [0xFFFF] Interrupt enable register.
    pub ie_reg: Flags,
}
impl MemoryBus {
    /// Create a new [MemoryBus].
    pub fn new() -> Self {
        Self {
            cart: None,
            cart_rom_0: [0x00; 0x4000],
            cart_rom_1: [0x00; 0x4000],
            cart_rom_2: [0x00; 0x4000],
            cart_rom_3: [0x00; 0x4000],
            cart_rom_5: [0x00; 0x4000],
            current_rom_bank: 1,
            vram: [0x00; 0x2000],
            eram: [0x00; 0x2000],
            wram: [0x00; 0x2000],
            wram_echo: [0x00; 0x1E00],
            oam: [0x00; 0x00A0],
            io_regs: IORegisters::new(),
            hram: [0x00; 0x007F],
            ie_reg: Flags::new(0b0000_0000),
        }
    }

    /// Perform one full cycle. Return the PPU t-cycles.
    pub fn cycle(&mut self, t_cycles: u32) -> u32 {
        // Cycle timer.
        self.io_regs.timer_cycle(t_cycles);

        // TODO check for joypad interrupts

        // Cycle PPU.
        // Set appropriate tile maps from VRAM.
        let bg_map_start = self.io_regs.ppu.bg_map_start() as usize;
        let win_map_start = self.io_regs.ppu.win_map_start() as usize;
        // Set appropriate tile data from VRAM.
        let tile_data_start = self.io_regs.ppu.tile_data_start() as usize;
        self.io_regs.ppu.cycle(
            t_cycles,
            &self.vram[bg_map_start..(bg_map_start + 0x0400)]
                .try_into()
                .unwrap(),
            &self.vram[win_map_start..(win_map_start + 0x0400)]
                .try_into()
                .unwrap(),
            &self.vram[tile_data_start..(tile_data_start + 0x2000)]
                .try_into()
                .unwrap(),
        );
        // Set IF register to accomodate any PPU-triggered interrupts.
        self.io_regs.interrupt_flags.write_byte(
            self.io_regs.interrupt_flags.read_byte() | self.io_regs.ppu.interrupt_flags.read_byte(),
        );
        self.io_regs.ppu.interrupt_flags.write_byte(0b0000_0000);

        // TODO cycle sound

        // TODO check for serial interrupts

        t_cycles
    }

    /// Match the currently-selected ROM bank.
    fn match_rom_bank(&self) -> &[u8; 0x4000] {
        match self.current_rom_bank {
            5 => &self.cart_rom_5,
            3 => &self.cart_rom_3,
            2 => &self.cart_rom_2,
            _ => &self.cart_rom_1,
        }
    }

    /// Match the currently-selected ROM bank.
    fn match_rom_bank_mut(&mut self) -> &mut [u8; 0x4000] {
        match self.current_rom_bank {
            5 => &mut self.cart_rom_5,
            3 => &mut self.cart_rom_3,
            2 => &mut self.cart_rom_2,
            _ => &mut self.cart_rom_1,
        }
    }

    /// Read the byte at the given address.
    // TODO enforce read restrictions
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.cart_rom_0[address as usize],
            0x4000..=0x7FFF => self.match_rom_bank()[address as usize - 0x4000],
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xA000..=0xBFFF => self.eram[address as usize - 0xA000],
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000],
            0xE000..=0xFDFF => self.wram_echo[address as usize - 0xE000],
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
    // TODO enforce write restrictions
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            0x0000..=0x3FFF => self.cart_rom_0[address as usize] = byte,
            0x4000..=0x7FFF => self.match_rom_bank_mut()[address as usize - 0x4000] = byte,
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = byte,
            0xA000..=0xBFFF => self.eram[address as usize - 0xA000] = byte,
            0xC000..=0xDFFF => {
                self.wram[address as usize - 0xC000] = byte;
                // Echo RAM only mirrors 0xC000..=0xDDFF.
                if address <= 0xDDFF {
                    self.wram_echo[address as usize - 0xC000] = byte;
                }
            }
            // Writing to wram_echo is prohibited
            0xE000..=0xFDFF => panic!("Illegal write attempt to Echo RAM @ {:#04X}", address),
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

    /// Load a [Cartridge] from a given file path.
    pub fn load_cart(&mut self, filepath: &str) {
        let cart = match Cartridge::from_file(filepath) {
            Some(cart) => cart,
            _ => {
                self.cart = None;
                return;
            }
        };

        let mut padded_data: [u8; 0x10000] = [0x00; 0x10000];
        if cart.data().len() >= padded_data.len() {
            padded_data.copy_from_slice(&cart.data()[0x0000..=0xFFFF]);
        } else {
            padded_data[..cart.data().len()].copy_from_slice(cart.data());
        }

        self.cart_rom_0
            .copy_from_slice(&padded_data[0x0000..=0x3FFF]);
        self.cart_rom_1
            .copy_from_slice(&padded_data[0x4000..=0x7FFF]);
        self.cart_rom_2
            .copy_from_slice(&padded_data[0x8000..=0xBFFF]);
        self.cart_rom_3
            .copy_from_slice(&padded_data[0xC000..=0xFFFF]);

        // Load boot ROM
        // TODO handle relative paths
        if let Some(boot_rom) = cart.boot_rom_data() {
            self.cart_rom_0[..0x0100].copy_from_slice(boot_rom)
        }

        self.cart = Some(cart);
    }

    /// Get the loaded [Cartridge].
    pub fn cart(&self) -> Option<&Cartridge> {
        self.cart.as_ref()
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

    #[test]
    fn test_rw_rom0() {
        let mut mb = MemoryBus::new();

        mb.write_byte(0x3FFF, 0xAB);
        assert_eq!(mb.read_byte(0x3FFF), 0xAB);
        assert_eq!(mb.cart_rom_0[0x3FFF], 0xAB);
    }

    #[test]
    fn test_rw_romn() {
        let mut mb = MemoryBus::new();

        mb.write_byte(0x4000, 0xAB);
        assert_eq!(mb.read_byte(0x4000), 0xAB);
        assert_eq!(mb.cart_rom_1[0x0000], 0xAB);

        mb.current_rom_bank = 2;
        mb.write_byte(0x4000, 0xCD);
        assert_eq!(mb.read_byte(0x4000), 0xCD);
        assert_eq!(mb.cart_rom_2[0x0000], 0xCD);
        assert_eq!(mb.cart_rom_1[0x0000], 0xAB);

        mb.current_rom_bank = 3;
        mb.write_byte(0x4000, 0xEF);
        assert_eq!(mb.read_byte(0x4000), 0xEF);
        assert_eq!(mb.cart_rom_3[0x0000], 0xEF);
        assert_eq!(mb.cart_rom_1[0x0000], 0xAB);
        assert_eq!(mb.cart_rom_2[0x0000], 0xCD);

        mb.current_rom_bank = 5;
        mb.write_byte(0x4000, 0x12);
        assert_eq!(mb.read_byte(0x4000), 0x12);
        assert_eq!(mb.cart_rom_5[0x0000], 0x12);
        assert_eq!(mb.cart_rom_1[0x0000], 0xAB);
        assert_eq!(mb.cart_rom_2[0x0000], 0xCD);
        assert_eq!(mb.cart_rom_3[0x0000], 0xEF);

        mb.current_rom_bank = 1;
        assert_eq!(mb.read_byte(0x4000), 0xAB);
        mb.current_rom_bank = 2;
        assert_eq!(mb.read_byte(0x4000), 0xCD);
        mb.current_rom_bank = 3;
        assert_eq!(mb.read_byte(0x4000), 0xEF);
        mb.current_rom_bank = 5;
        assert_eq!(mb.read_byte(0x4000), 0x12);
    }

    #[test]
    fn test_rw_vram() {
        let mut mb = MemoryBus::new();

        mb.write_byte(0x8000, 0xAB);
        mb.write_byte(0x9FFF, 0xCD);
        assert_eq!(mb.read_byte(0x8000), 0xAB);
        assert_eq!(mb.vram[0x0000], 0xAB);
        assert_eq!(mb.read_byte(0x9FFF), 0xCD);
        assert_eq!(mb.vram[0x1FFF], 0xCD);
    }

    #[test]
    fn test_rw_eram() {
        let mut mb = MemoryBus::new();

        mb.write_byte(0xA000, 0xAB);
        mb.write_byte(0xBFFF, 0xCD);
        assert_eq!(mb.read_byte(0xA000), 0xAB);
        assert_eq!(mb.eram[0x0000], 0xAB);
        assert_eq!(mb.read_byte(0xBFFF), 0xCD);
        assert_eq!(mb.eram[0x1FFF], 0xCD);
    }

    #[test]
    fn test_rw_wram() {
        let mut mb = MemoryBus::new();

        mb.write_byte(0xC000, 0xAB);
        assert_eq!(mb.read_byte(0xC000), 0xAB);
        assert_eq!(mb.wram[0x0000], 0xAB);
        assert_eq!(mb.wram_echo[0x0000], 0xAB);

        mb.write_byte(0xDFFF, 0xCD);
        assert_eq!(mb.read_byte(0xDFFF), 0xCD);
        assert_eq!(mb.wram[0x1FFF], 0xCD);

        mb.write_byte(0xDDFF, 0x12);
        assert_eq!(mb.read_byte(0xDDFF), 0x12);
        assert_eq!(mb.wram[0x1DFF], 0x12);
        assert_eq!(mb.wram_echo[0x1DFF], 0x12);
    }

    #[test]
    fn test_rw_oam() {
        let mut mb = MemoryBus::new();

        mb.write_byte(0xFE00, 0xAB);
        mb.write_byte(0xFE9F, 0xCD);
        assert_eq!(mb.read_byte(0xFE00), 0xAB);
        assert_eq!(mb.oam[0x0000], 0xAB);
        assert_eq!(mb.read_byte(0xFE9F), 0xCD);
        assert_eq!(mb.oam[0x009F], 0xCD);
    }

    #[test]
    fn test_rw_io_regs() {
        let mut mb = MemoryBus::new();

        mb.write_byte(0xFF00, 0xAB);
        mb.write_byte(0xFF70, 0xCD);
        assert_eq!(mb.read_byte(0xFF00), 0xAB);
        assert_eq!(mb.io_regs.read_byte(0x0000), 0xAB);
        assert_eq!(mb.read_byte(0xFF70), 0xCD);
        assert_eq!(mb.io_regs.read_byte(0x0070), 0xCD);
    }

    #[test]
    fn test_rw_hram() {
        let mut mb = MemoryBus::new();

        mb.write_byte(0xFF80, 0xAB);
        mb.write_byte(0xFFFE, 0xCD);
        assert_eq!(mb.read_byte(0xFF80), 0xAB);
        assert_eq!(mb.hram[0x0000], 0xAB);
        assert_eq!(mb.read_byte(0xFFFE), 0xCD);
        assert_eq!(mb.hram[0x007E], 0xCD);
    }
}
