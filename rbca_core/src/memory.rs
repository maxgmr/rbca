//! Functionality related to emulator memory.
use std::default::Default;

use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    cartridge::{self, Cartridge},
    io_registers::IORegisters,
    Flags,
};

// TODO replace with args
const USE_BOOT_ROM: bool = false;
const USE_CARTRIDGE: bool = true;

const BOOT_ROM_PATH: &str = "dmg-boot.bin";

/// Memory bus of the Game Boy.
#[derive(Debug)]
pub struct MemoryBus {
    /// The [Cartridge] loaded from the computer.
    pub cart: Option<Box<dyn Cartridge>>,
    /// The boot ROM hard-coded into the device.
    pub boot_rom: Option<Box<dyn Cartridge>>,
    /// Selects whether to read/write from/to the boot ROM in the first 0x0100 addresses
    pub boot_rom_active: bool,
    /// [0x8000-0x9FFF] Video RAM. Background & sprite data.
    pub vram: [u8; 0x2000],
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
    pub fn new<P: AsRef<Utf8Path>>(filepath: Option<P>) -> Self {
        Self {
            cart: if let Some(path) = filepath {
                Some(cartridge::load_cartridge(path.as_ref()))
            } else {
                None
            },
            boot_rom: if USE_BOOT_ROM {
                Some(cartridge::load_cartridge(Utf8PathBuf::from(BOOT_ROM_PATH)))
            } else {
                None
            },
            boot_rom_active: USE_BOOT_ROM,
            vram: [0x00; 0x2000],
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

    /// Read the byte at the given address.
    // TODO enforce read restrictions
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => {
                if let Some(cart) = self.cart {
                    cart.read_rom(address)
                } else {
                    0xFF
                }
            }
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xA000..=0xBFFF => {
                if let Some(cart) = self.cart {
                    cart.read_ram(address)
                } else {
                    0xFF
                }
            }
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000],
            0xE000..=0xFDFF => self.wram_echo[address as usize - 0xE000],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFEA0..=0xFEFF => 0xFF,
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
            0x0000..=0x7FFF => {
                if let Some(cart) = self.cart {
                    cart.write_rom(address, byte)
                }
            }
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = byte,
            0xA000..=0xBFFF => {
                if let Some(cart) = self.cart {
                    cart.write_ram(address, byte)
                }
            }
            0xC000..=0xDFFF => {
                self.wram[address as usize - 0xC000] = byte;
                // Echo RAM only mirrors 0xC000..=0xDDFF.
                if address <= 0xDDFF {
                    self.wram_echo[address as usize - 0xC000] = byte;
                }
            }
            // Writing to wram_echo is prohibited
            0xE000..=0xFDFF => {}
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = byte,
            0xFEA0..=0xFEFF => {}
            // OAM DMA transfer
            0xFF46 => self.oam_dma_transfer(byte),
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
        let cart = cartridge::load_cartridge(filepath);
        self.cart = Some(cart);
    }

    /// Get the loaded [Cartridge].
    pub fn cart(&self) -> Option<&Box<dyn Cartridge>> {
        self.cart.as_ref()
    }

    /// Perform DMA transfer from ROM/RAM to OAM.
    fn oam_dma_transfer(&mut self, byte: u8) {
        let transfer_source_addr = (byte as u16) << 8;
        for i in 0..0xA0 {
            let current_byte = self.read_byte(transfer_source_addr + i);
            self.write_byte(0xFE00 + i, current_byte);
        }
    }
}
impl Default for MemoryBus {
    fn default() -> Self {
        Self::new(None::<Utf8PathBuf>)
    }
}
