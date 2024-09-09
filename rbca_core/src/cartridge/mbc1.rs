use super::{CartFeatures, Cartridge};

/// An MBC1 cartridge.
#[derive(Debug)]
pub struct CartMBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    cart_features: CartFeatures,
    ram_enable: bool,
    rom_bank_number: u8,
    ram_bank_number: u8,
    /// false = 0 = simple, true = 1 = advanced
    banking_mode_select: bool,
}
impl CartMBC1 {
    pub fn new(data: Vec<u8>, cart_features: CartFeatures) -> Self {
        let mut mbc1 = Self {
            rom: data,
            ram: vec![],
            cart_features,
            ram_enable: false,
            rom_bank_number: 0x00,
            ram_bank_number: 0x00,
            banking_mode_select: false,
        };
        // Allocate RAM based on RAM size denoted in cartridge header
        mbc1.ram = std::iter::repeat(0x00_u8)
            .take(mbc1.ram_size() as usize)
            .collect();
        mbc1
    }

    fn num_banks(&self) -> u8 {
        (self.rom_size() / 0x4000).try_into().unwrap()
    }

    fn get_rom_bank_number(&self) -> u8 {
        match self.rom_bank_number & 0b0001_1111 {
            // If last 5 bits of register are 0 or 1, ROM bank number = 1.
            0x00 | 0x01 => 0b0000_0001,
            // For >5 bit bank number, RAM bank number is used to provide additional 2 bits. These
            // bits are ignored for the 00->01 translation.
            rbn if rbn > 5 => (self.ram_bank_number & 0b0000_0011) | rbn,
            // If register is set to a higher value than the number of banks in the cart, the bank
            // number is masked to the required number of bits.
            rbn if rbn > self.num_banks() => rbn & (self.num_banks() - 1),
            rbn => rbn,
        }
    }

    fn set_rom_bank_number(&mut self, value: u8) {
        self.rom_bank_number = value & 0b0001_1111;
    }

    fn get_ram_bank_number(&self) -> u8 {
        self.ram_bank_number & 0b0000_0011
    }

    fn set_ram_bank_number(&mut self, value: u8) {
        self.ram_bank_number = value & 0b0000_0011;
    }

    fn internal_addr(&self, address: u16) -> usize {
        let mut result: u32 = 0x0000_0000;
        match address {
            0x0000..=0x3FFF => {
                if self.banking_mode_select {
                    result |= (self.get_ram_bank_number() as u32) << 19;
                }
                result |= (address as u32) & 0b0011_1111_1111_1111;
            }
            0x4000..=0x7FFF => {
                result |= (self.get_ram_bank_number() as u32) << 19;
                result |= (self.get_rom_bank_number() as u32) << 14;
                result |= (address as u32) & 0b0011_1111_1111_1111;
            }
            0xA000..=0xBFFF => {
                if self.banking_mode_select {
                    result |= (self.get_ram_bank_number() as u32) << 13;
                }
                result |= (address as u32) & 0b0001_1111_1111_1111;
            }
            _ => panic!(
                "Tried to get internal cart address of address {:#06X}.",
                address
            ),
        };
        result as usize
    }
}
impl Cartridge for CartMBC1 {
    fn rom(&self) -> &[u8] {
        &self.rom
    }

    fn cart_features(&self) -> &CartFeatures {
        &self.cart_features
    }

    fn read_rom(&self, address: u16) -> u8 {
        *self
            .rom
            .get(self.internal_addr(address))
            .unwrap_or(&0xFF_u8)
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enable {
            return 0xFF_u8;
        }
        self.ram[self.internal_addr(address)]
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // RAM enable
            0x0000..=0x1FFF => self.ram_enable = (value & 0x0F) == 0x0A,
            // ROM bank number
            0x2000..=0x3FFF => self.set_rom_bank_number(value),
            // RAM bank number
            0x4000..=0x5FFF => self.set_ram_bank_number(value),
            // Banking mode select
            0x6000..=0x7FFF => self.banking_mode_select = (value & 0b0000_0001) == 0x01,
            _ => panic!("MBC1 Cart: Cannot write to {:#06X}", address),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enable {
            return;
        }
        let addr = self.internal_addr(address);
        self.ram[addr] = value;
    }
}
