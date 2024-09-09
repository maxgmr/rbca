use super::{CartFeatures, Cartridge, BYTES_IN_KIB};

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
    pub fn new(data: Vec<u8>) -> Self {
        let cart_features = CartFeatures::from_data(&data);

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

        // Validate proper parameters
        // Cart features include MBC1
        if !mbc1.cart_features.mbc1 {
            panic!("New MBC1: According to the header, this cartridge is not an MBC1. Cart type from header: {}", mbc1.cart_features);
        }

        // Allocated ROM size matches ROM size in header
        if mbc1.rom().len() != mbc1.rom_size() as usize {
            panic!("New MBC1: ROM size in header does not match provided data ({} bytes of ROM data != {} bytes in header)", mbc1.rom().len(), mbc1.rom_size());
        }

        // Max ROM size
        if mbc1.rom().len() > (2048 * BYTES_IN_KIB) as usize {
            panic!(
                "New MBC1: ROM size too big ({} MiB > 2 MiB)",
                mbc1.rom().len() / 1024 / (BYTES_IN_KIB as usize)
            );
        }
        // Max RAM size
        if mbc1.ram.len() > (32 * BYTES_IN_KIB) as usize {
            panic!(
                "New MBC1: RAM size too big ({} KiB > 32 KiB)",
                mbc1.ram.len() / (BYTES_IN_KIB as usize)
            );
        }
        // Max ROM size + max RAM size
        if (mbc1.rom().len() > (512 * BYTES_IN_KIB) as usize)
            && (mbc1.ram.len() > (8 * BYTES_IN_KIB) as usize)
        {
            panic!("New MBC1: MBC1 with >512 KiB of ROM can only have up to 8 KiB of RAM ({} KiB > 8 KiB)", mbc1.ram.len() / (BYTES_IN_KIB as usize));
        }

        mbc1
    }

    fn num_rom_banks(&self) -> u8 {
        (self.rom_size() / 0x4000).try_into().unwrap()
    }

    fn num_ram_banks(&self) -> u8 {
        (self.ram_size() / 0x2000).try_into().unwrap()
    }

    fn get_rom_bank_number(&self) -> u8 {
        match self.rom_bank_number & 0b0001_1111 {
            // If last 5 bits of register = 0 or 1, ROM bank number = 1.
            0x00 | 0x01 => 0b0000_0001,
            // If register is set to a higher value than the number of banks in the cart, the bank
            // number is masked to the required number of bits.
            rbn if rbn > (self.num_rom_banks() - 1) => rbn & (self.num_rom_banks() - 1),
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

    #[cfg(test)]
    fn new_test(
        has_ram: bool,
        has_battery: bool,
        num_rom_banks: usize,
        num_ram_banks: usize,
    ) -> Self {
        use super::{checksum_fn, BYTES_IN_KIB, LOGO};

        let (rom_bytes, rom_size_value) = match num_rom_banks {
            2 => (32 * BYTES_IN_KIB, 0x00_u8),
            4 => (64 * BYTES_IN_KIB, 0x01_u8),
            8 => (128 * BYTES_IN_KIB, 0x02_u8),
            16 => (256 * BYTES_IN_KIB, 0x03_u8),
            32 => (512 * BYTES_IN_KIB, 0x04_u8),
            64 => (1024 * BYTES_IN_KIB, 0x05_u8),
            128 => (2048 * BYTES_IN_KIB, 0x06_u8),
            _ => panic!("Illegal value for num_rom_banks ({num_rom_banks})"),
        };

        let ram_size_value = match (num_rom_banks, has_ram, num_ram_banks) {
            (_, false, 0) => 0x00_u8,
            (_, true, 1) => 0x02_u8,
            (nrb, true, 4) if nrb <= 32 => 0x03_u8,
            _ => panic!(
                "Illegal value combo for num_rom_banks, has_ram, num_ram_banks ({num_rom_banks}, {has_ram}, {num_ram_banks})"
            ),
        };

        let mut data: Vec<u8> = std::iter::repeat(0x00_u8)
            .take(rom_bytes as usize)
            .collect();

        data[0x0101..=0x0103].copy_from_slice(&[0xC3, 0x50, 0x01]);
        data[0x0104..=0x0133].copy_from_slice(&LOGO);
        data[0x0134..=0x013E].copy_from_slice(&[
            0x54, 0x65, 0x73, 0x74, 0x20, 0x43, 0x61, 0x72, 0x74, 0x00, 0x00,
        ]);
        data[0x147] = match (has_ram, has_battery) {
            (true, true) => 0x03,
            (true, false) => 0x02,
            (false, false) => 0x01,
            _ => panic!("Any MBC1 with a battery must have RAM."),
        };
        data[0x148] = rom_size_value;
        data[0x149] = ram_size_value;
        data[0x14D] = checksum_fn(&data);

        CartMBC1::new(data)
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
            0x4000..=0x5FFF => {
                if self.num_rom_banks() > 32 || self.num_ram_banks() > 1 {
                    self.set_ram_bank_number(value)
                }
            }
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    #[should_panic]
    fn test_bad_rom_size_header() {
        let mut data: Vec<u8> = std::iter::repeat(0x00_u8)
            .take((64 * BYTES_IN_KIB) as usize)
            .collect();
        data[0x0147] = 0x01;
        data[0x0148] = 0x02;
        let _ = CartMBC1::new(data);
    }

    #[test]
    #[should_panic]
    fn test_rom_too_large() {
        let mut data: Vec<u8> = std::iter::repeat(0x00_u8)
            .take((4 * 1024 * BYTES_IN_KIB) as usize)
            .collect();
        data[0x0147] = 0x01;
        data[0x0148] = 0x07;
        let _ = CartMBC1::new(data);
    }

    #[test]
    #[should_panic]
    fn test_ram_too_large() {
        let mut data: Vec<u8> = std::iter::repeat(0x00_u8)
            .take((64 * BYTES_IN_KIB) as usize)
            .collect();
        data[0x0147] = 0x03;
        data[0x0148] = 0x01;
        data[0x0149] = 0x05;
        let _ = CartMBC1::new(data);
    }

    #[test]
    #[should_panic]
    fn test_rom_ram_too_large() {
        let mut data: Vec<u8> = std::iter::repeat(0x00_u8)
            .take((1024 * BYTES_IN_KIB) as usize)
            .collect();
        data[0x0147] = 0x03;
        data[0x0148] = 0x05;
        data[0x0149] = 0x03;
        let _ = CartMBC1::new(data);
    }

    #[test]
    #[should_panic]
    fn test_bad_header() {
        let mut data: Vec<u8> = std::iter::repeat(0x00_u8)
            .take((1024 * BYTES_IN_KIB) as usize)
            .collect();
        data[0x0147] = 0x00;
        data[0x0148] = 0x00;
        data[0x0149] = 0x00;
        let _ = CartMBC1::new(data);
    }

    #[test]
    fn test_new() {
        let mbc1 = CartMBC1::new_test(false, false, 4, 0);

        assert_eq!(mbc1.logo(), super::super::LOGO);
        assert!(mbc1.validate_logo());

        assert_eq!(mbc1.num_rom_banks(), 4);
        assert_eq!(mbc1.num_ram_banks(), 0);

        assert!(mbc1.cart_features().mbc1);
        assert!(!mbc1.cart_features().ram);
        assert!(!mbc1.cart_features().battery);

        assert_eq!(mbc1.title(), "Test Cart\0\0\0\0\0\0\0");

        assert_eq!(mbc1.rom_size(), 65536);
        assert_eq!(mbc1.ram_size(), 0);

        assert!(mbc1.validate_checksum().is_none());
    }

    #[test]
    fn test_bank_numbers() {
        let mut mbc1 = CartMBC1::new_test(true, false, 16, 4);

        mbc1.set_rom_bank_number(0b0000_0000);
        assert_eq!(mbc1.get_rom_bank_number(), 1);

        mbc1.set_rom_bank_number(0b1110_0000);
        assert_eq!(mbc1.get_rom_bank_number(), 1);

        mbc1.set_rom_bank_number(0b1111_1111);
        assert_eq!(mbc1.get_rom_bank_number(), 15);

        // Can map bank 0 to 0x4000-0x7FFF region on <=256KiB carts
        mbc1.set_rom_bank_number(0b1111_0000);
        assert_eq!(mbc1.get_rom_bank_number(), 0);

        mbc1.set_ram_bank_number(0b1010_1011);
        assert_eq!(mbc1.get_ram_bank_number(), 3);
        mbc1.set_ram_bank_number(0b0000_0010);
        assert_eq!(mbc1.get_ram_bank_number(), 2);
        mbc1.set_ram_bank_number(0b1111_1101);
        assert_eq!(mbc1.get_ram_bank_number(), 1);
        mbc1.set_ram_bank_number(0b0010_0100);
        assert_eq!(mbc1.get_ram_bank_number(), 0);
    }

    #[test]
    fn test_ram() {
        let mut mbc1 = CartMBC1::new_test(true, true, 32, 4);
        mbc1.ram[0x0000] = 0x89;
        mbc1.ram[0x1234] = 0xAB;
        mbc1.ram[0x1FFF] = 0xCD;

        mbc1.write_rom(0x0000, 0xFB);
        assert!(!mbc1.ram_enable);
        assert_eq!(mbc1.read_ram(0xA000), 0xFF);

        mbc1.write_rom(0x0000, 0xEA);
        assert!(mbc1.ram_enable);
        assert_eq!(mbc1.read_ram(0xA000), 0x89);

        mbc1.write_rom(0x1FFF, 0x00);
        assert!(!mbc1.ram_enable);
        assert_eq!(mbc1.read_ram(0xB234), 0xFF);
        mbc1.write_ram(0xB234, 0x12);

        mbc1.write_rom(0x1FCF, 0x0A);
        assert!(mbc1.ram_enable);
        assert_eq!(mbc1.read_ram(0xB234), 0xAB);
        assert_eq!(mbc1.read_ram(0xBFFF), 0xCD);
    }

    #[test]
    fn test_rom_bank_switching() {
        let mut mbc1 = CartMBC1::new_test(true, true, 128, 1);
        mbc1.banking_mode_select = false;
        for bank_num in 0..128 {
            let addr_offset = match bank_num % 3 {
                0 => 0x0000,
                1 => 0x2FAB,
                2 => 0x3FFF,
                _ => unreachable!(),
            };
            mbc1.rom[(bank_num * 0x4000) + addr_offset] = bank_num as u8;
        }

        // Set first byte of bank 1 to 0xFF for special 0 case
        mbc1.rom[0x4000] = 0xFF;
        // Set specific bytes of banks 0x21, 0x41, and 0x61 to test special 0x20, 0x40, and 0x60
        // behaviour
        mbc1.rom[(0x4000 * 0x21) + 0x0F] = 0x20;
        mbc1.rom[(0x4000 * 0x41) + 0x0F] = 0x40;
        mbc1.rom[(0x4000 * 0x61) + 0x0F] = 0x60;

        for bank_num in 0..128 {
            let addr_offset = match bank_num % 3 {
                0 => 0x0000,
                1 => 0x2FAB,
                2 => 0x3FFF,
                _ => unreachable!(),
            };

            // Set ROM bank number
            mbc1.write_rom(if bank_num % 2 != 0 { 0x2000 } else { 0x3FFF }, bank_num);
            mbc1.write_rom(
                if bank_num % 2 != 0 { 0x4000 } else { 0x5FFF },
                bank_num >> 5,
            );

            // Special cases
            if bank_num == 0 {
                // Should access bank 1
                assert_eq!(mbc1.read_rom(0x4000), 0xFF);
            } else if bank_num == 0x20 || bank_num == 0x40 || bank_num == 0x60 {
                // Should access banks 0x21, 0x41, or 0x61 respectively
                // Set mode 0
                mbc1.write_rom(0x6000, 0x00);
                assert!(!mbc1.banking_mode_select);
                assert_eq!(mbc1.read_rom(0x400F), bank_num);
                // Should be able to access the actual banks @ 0x0000-0x3FFF if mode 1
                // Set mode 1
                mbc1.write_rom(0x7FFF, 0xA1);
                assert!(mbc1.banking_mode_select);
                assert_eq!(mbc1.read_rom(addr_offset), bank_num);
                // Set mode 0
                mbc1.write_rom(0x7FAB, 0x00);
                assert_eq!(mbc1.read_rom(addr_offset), 0x00);
            // Normal case
            } else {
                // Set mode 0
                mbc1.write_rom(0x7FAB, 0x00);
                assert_eq!(mbc1.read_rom(0x4000 + addr_offset), bank_num);
                // Set mode 1
                mbc1.write_rom(0x7FAB, 0x01);
                assert_eq!(mbc1.read_rom(0x4000 + addr_offset), bank_num);
                // Set mode 0
                mbc1.write_rom(0x7FAB, 0x00);
            }
        }
    }

    #[test]
    fn test_ram_bank_switching() {
        let mut mbc1 = CartMBC1::new_test(true, false, 8, 4);

        // set values to find
        mbc1.ram[0x0000] = 0xAB;
        for bank_num in 1..4 {
            mbc1.ram[(0x2000 * (bank_num as usize)) + (bank_num as usize)] = bank_num;
        }

        for bank_num in 0..4 {
            // set mode 0
            mbc1.write_rom(0x6FAB, 0x00);
            assert!(!mbc1.banking_mode_select);

            // switch RAM bank num
            mbc1.write_rom(0x4000 + (0x07FF * bank_num), bank_num.try_into().unwrap());

            // disable RAM
            mbc1.write_rom(0x07FF * bank_num, 0xBB);
            assert!(!mbc1.ram_enable);

            // ensure 0xFF is read
            assert_eq!(mbc1.read_ram(0xA000 + bank_num), 0xFF);

            // enable RAM
            mbc1.write_rom(0x07FF * bank_num, 0xFA);
            assert!(mbc1.ram_enable);

            // ensure ram[0x0000] is read in mode 0
            assert_eq!(mbc1.read_ram(0xA000), 0xAB);

            // set mode 1
            mbc1.write_rom(0x7FAB, 0x01);
            assert!(mbc1.banking_mode_select);

            // ensure the correct value is read
            if bank_num == 0 {
                assert_eq!(mbc1.read_ram(0xA000), 0xAB);
            } else {
                assert_eq!(
                    mbc1.read_ram(0xA000 + bank_num),
                    bank_num.try_into().unwrap()
                );
            }
        }
    }

    #[test]
    fn test_useless_ram_bank_number() {
        let mut mbc1 = CartMBC1::new_test(true, true, 32, 1);
        assert_eq!(mbc1.get_ram_bank_number(), 0);

        // set values to find
        for bank_num in 1..32 {
            mbc1.ram[bank_num as usize] = bank_num;
        }

        // enable RAM
        mbc1.write_rom(0x1FAB, 0x0A);

        for bank_num in 1..32 {
            // set RAM bank num (this should do nothing because there's only one RAM bank)
            mbc1.write_rom(0x4000, bank_num.try_into().unwrap());
            assert_eq!(mbc1.get_ram_bank_number(), 0);

            // set mode 1
            mbc1.write_rom(0x7FAB, 0x01);
            assert!(mbc1.banking_mode_select);

            // ensure that the value is being read from the same (only) bank
            assert_eq!(mbc1.read_ram(0xA000 + bank_num), bank_num as u8);
        }
    }
}
