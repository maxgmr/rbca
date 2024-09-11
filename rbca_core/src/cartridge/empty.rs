use super::{CartFeatures, Cartridge};

/// An empty cartridge. Allows for read & write testing when no cartridge is inserted.
/// Acts as a ROM-only cartridge (data from 0x0000..=0x7FFF), but it can be read and written to
/// everywhere.
#[derive(Debug)]
pub struct CartEmpty {
    rom: Vec<u8>,
    cart_features: CartFeatures,
}
impl CartEmpty {
    pub fn new() -> Self {
        Self {
            rom: std::iter::repeat(0x00_u8).take(0x8000).collect(),
            cart_features: CartFeatures::default(),
        }
    }
}
impl Cartridge for CartEmpty {
    fn rom(&self) -> &[u8] {
        &self.rom
    }

    fn cart_features(&self) -> &CartFeatures {
        &self.cart_features
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn read_rom(&self, address: u16) -> u8 {
        if (address as usize) < self.rom.len() {
            self.rom[address as usize]
        } else {
            0xFF
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.read_rom(address)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        if (address as usize) < self.rom.len() {
            self.rom[address as usize] = value
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.write_rom(address, value)
    }
}
