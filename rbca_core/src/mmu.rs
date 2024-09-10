use std::{fs::File, io::Read};

use camino::Utf8Path;

use crate::{
    cartridge::{self, CartEmpty, Cartridge},
    Audio, Flags, FlagsEnum, Timer, PPU,
};

/// Memory management unit. Routes reads and writes and controls device state.
#[derive(Debug)]
pub struct Mmu {
    /// The [Cartridge] loaded from the computer.
    pub cart: Box<dyn Cartridge>,
    /// The boot ROM.
    boot_rom: Option<[u8; 0x0100]>,
    /// Work RAM.
    wram: [u8; 0x2000],
    /// Echo RAM. Mirror of 0xC000-0xDDFF.
    eram: [u8; 0x1E00],
    /// Joypad input.
    joypad: Flags,
    /// Serial transfer data.
    serial_data: u8,
    /// Serial transfer control.
    serial_control: Flags,
    /// Timer.
    timer: Timer,
    /// Interrupt flags register.
    if_reg: Flags,
    /// Audio.
    audio: Audio,
    /// Pixel processing unit.
    ppu: PPU,
    /// Diable boot ROM flag.
    disable_boot_rom: u8,
    /// High RAM.
    hram: [u8; 0x007F],
    /// Interrupt enable register.
    ie_reg: Flags,
}
impl Mmu {
    /// Create a new [Mmu] with a cartridge and a boot ROM.
    pub fn new_boot_cart<P: AsRef<Utf8Path>>(cart_path: P, boot_rom_path: P) -> Self {
        Self::new_helper(
            cartridge::load_cartridge(cart_path),
            Some(load_boot_rom(boot_rom_path)),
        )
    }

    /// Create a new [Mmu] without loading a cartridge.
    pub fn new_boot<P: AsRef<Utf8Path>>(boot_rom_path: P) -> Self {
        Self::new_helper(
            Box::new(CartEmpty::new()),
            Some(load_boot_rom(boot_rom_path)),
        )
    }

    /// Create a new [Mmu] without a boot ROM.
    pub fn new_cart<P: AsRef<Utf8Path>>(cart_path: P) -> Self {
        Self::new_helper(cartridge::load_cartridge(cart_path), None)
    }

    /// Create a new [Mmu] without loading a cartridge or a boot ROM.
    pub fn new() -> Self {
        Self::new_helper(Box::new(CartEmpty::new()), None)
    }

    fn new_helper(cart: Box<dyn Cartridge>, boot_rom: Option<[u8; 0x0100]>) -> Self {
        let have_boot_rom = boot_rom.is_some();
        Self {
            cart,
            boot_rom,
            wram: [0x00; 0x2000],
            eram: [0x00; 0x1E00],
            joypad: Flags::new(0b0011_1111),
            serial_data: 0x00,
            serial_control: Flags::new(0b0000_0000),
            timer: Timer::new(),
            if_reg: Flags::new(0b0000_0000),
            audio: Audio::new(),
            ppu: PPU::new(),
            disable_boot_rom: if have_boot_rom { 0x00 } else { 0x01 },
            hram: [0x00; 0x007F],
            ie_reg: Flags::new(0b0000_0000),
        }
    }

    /// Directly read the byte at the given address.
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x00FF => {
                if self.read_byte(0xFF50) != 0 {
                    return self.cart.read_rom(address);
                }
                if let Some(boot_rom) = self.boot_rom {
                    return boot_rom[address as usize];
                }
                self.cart.read_rom(address)
            }
            0x0100..=0x7FFF => self.cart.read_rom(address),
            0x8000..=0x9FFF => self.ppu.read_byte(address),
            0xA000..=0xBFFF => self.cart.read_ram(address),
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000],
            0xE000..=0xFDFF => self.eram[address as usize - 0xE000],
            0xFE00..=0xFE9F => self.ppu.read_byte(address),
            0xFF00 => self.joypad.read_byte(),
            0xFF01 => self.serial_data,
            0xFF02 => self.serial_control.read_byte(),
            0xFF04..=0xFF07 => self.timer.read_byte(address),
            0xFF0F => self.if_reg.read_byte(),
            0xFF10..=0xFF3F => self.audio.read_byte(address),
            0xFF40..=0xFF4F => self.ppu.read_byte(address),
            0xFF50 => self.disable_boot_rom,
            0xFF51..=0xFF55 => self.ppu.read_byte(address),
            0xFF68..=0xFF6B => self.ppu.read_byte(address),
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
            0xFFFF => self.ie_reg.read_byte(),
            _ => 0xFF,
            // _ => panic!("MMU: read illegal address {:#06X}.", address),
        }
    }

    /// Read 2 bytes from the given (little-endian) start address.
    pub fn read_2_bytes(&mut self, start_address: u16) -> u16 {
        (self.read_byte(start_address) as u16) | ((self.read_byte(start_address + 1) as u16) << 8)
    }

    /// Directly write to the byte at the given address.
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x00FF => {
                if self.read_byte(0xFF50) != 0 {
                    self.cart.write_rom(address, value);
                }
            }
            0x0100..=0x7FFF => self.cart.write_rom(address, value),
            0x8000..=0x9FFF => self.ppu.write_byte(address, value),
            0xA000..=0xBFFF => self.cart.write_ram(address, value),
            0xC000..=0xDDFF => {
                self.wram[address as usize - 0xC000] = value;
                self.eram[address as usize - 0xC000] = value;
            }
            0xDE00..=0xDFFF => self.wram[address as usize - 0xC000] = value,
            0xE000..=0xFDFF => {}
            0xFE00..=0xFE9F => self.ppu.write_byte(address, value),
            0xFF00 => self.joypad.write_byte(value),
            0xFF01 => self.serial_data = value,
            0xFF02 => self.serial_control.write_byte(value),
            0xFF04..=0xFF07 => self.timer.write_byte(address, value),
            0xFF0F => self.if_reg.write_byte(value),
            0xFF10..=0xFF3F => self.audio.write_byte(address, value),
            0xFF40..=0xFF4F => self.ppu.write_byte(address, value),
            0xFF50 => self.disable_boot_rom = value,
            0xFF68..=0xFF6B => self.ppu.write_byte(address, value),
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = value,
            0xFFFF => self.ie_reg.write_byte(value),
            _ => {} // _ => panic!("MMU: write illegal address {:#06X}.", address),
        }
    }

    /// Write 2 (little-endian) bytes to a given start address.
    pub fn write_2_bytes(&mut self, start_address: u16, value: u16) {
        self.write_byte(start_address, (value & 0x00FF) as u8);
        self.write_byte(start_address + 1, (value >> 8) as u8);
    }

    /// Perform one full cycle, returning the PPU t-cycles.
    pub fn cycle(&mut self, t_cycles: u32) -> u32 {
        // Cycle the timer.
        self.timer.cycle(t_cycles);
        // Update IF register in the timer triggered any interrupts.
        self.if_reg |= self.timer.interrupt_flags;

        // TODO check for joypad interrupts.

        // Cycle the PPU.
        // TODO
        // let ppu_cycles = self.ppu.cycle(t_cycles);

        // TODO cycle sound.
        self.audio.cycle(t_cycles);

        // TODO check for serial interrupts.

        // TODO return PPU cycles.
        t_cycles
    }
}

fn load_boot_rom<P: AsRef<Utf8Path>>(filepath: P) -> [u8; 0x0100] {
    let mut file_buf = vec![];
    if let Err(e) = File::open(filepath.as_ref()).and_then(|mut f| f.read_to_end(&mut file_buf)) {
        panic!("{e}");
    }
    let mut boot_rom_data: [u8; 0x0100] = [0x00; 0x0100];
    boot_rom_data.copy_from_slice(&file_buf[..0x0100]);
    boot_rom_data
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

#[cfg(test)]
mod test_mmu;