//! Functionality related to Game Boy cartridges.

use std::{fs::File, io::Read};

const BYTES_IN_KIB: u32 = 128;

/// A Game Boy cartridge.
#[derive(Debug, Clone)]
pub struct Cartridge {
    ///  The boot ROM.
    boot_rom_data: Option<[u8; 0x100]>,
    /// The raw binary data stored on the cartridge.
    data: [u8; 0x10000],
    /// The hardware present on the cartridge.
    cart_features: CartFeatures,
}
impl Cartridge {
    /// Load a cartridge by loading a binary file at the given path.
    pub fn from_file(filepath: &str) -> Self {
        let mut data = vec![];
        File::open(filepath)
            .and_then(|mut f| f.read_to_end(&mut data))
            .expect("Could not read file.");
        let mut padded_data: [u8; 0x10000] = [0x00; 0x10000];
        padded_data[..data.len()].copy_from_slice(&data);

        let boot_rom_data = Self::load_boot_rom();

        let cart_features = CartFeatures::from_data(&padded_data);

        Self {
            boot_rom_data,
            data: padded_data,
            cart_features,
        }
    }

    /// Get the data stored on the cartridge.
    pub fn data(&self) -> &[u8; 0x10000] {
        &self.data
    }

    /// Get the boot ROM data.
    pub fn boot_rom_data(&self) -> Option<&[u8; 0x100]> {
        self.boot_rom_data.as_ref()
    }

    /// Get the cartridge features.
    pub fn cart_features(&self) -> &CartFeatures {
        &self.cart_features
    }

    /// Attempt to load the boot ROM.
    fn load_boot_rom() -> Option<[u8; 0x0100]> {
        let mut data = vec![];
        if File::open("dmg-boot.bin")
            .and_then(|mut f| f.read_to_end(&mut data))
            .is_err()
        {
            eprintln!("Warning: no boot ROM found.");
            return None;
        };
        let mut fixed_size_data: [u8; 0x0100] = [0x00; 0x0100];
        fixed_size_data.copy_from_slice(&data);
        Some(fixed_size_data)
    }

    /// Get the logo shown at startup.
    pub fn nin_logo(&self) -> &[u8] {
        &self.data[0x0104..=0x0133]
    }

    /// Get the title of the game (16-byte size for older cartridges).
    pub fn title_16(&self) -> &str {
        self.title_helper(0x0143)
    }
    /// Get the title of the game (15-byte size for newer DMG cartridges).
    pub fn title_15(&self) -> &str {
        self.title_helper(0x0142)
    }
    /// Get the title of the game (11-byte size for CGB & later cartridges).
    pub fn title_11(&self) -> &str {
        self.title_helper(0x013E)
    }
    fn title_helper(&self, end_index: u16) -> &str {
        std::str::from_utf8(&self.data[0x0134..=(end_index as usize)])
            .expect("Invalid UTF-8 sequence.")
    }

    /// Get the manufacturer code.
    pub fn manufacturer_code(&self) -> &[u8] {
        &self.data[0x013F..=0x0142]
    }

    /// Get the CGB flag.
    pub fn cgb_flag(&self) -> CgbFlag {
        match self.data[0x0143] {
            0b1000_0000 => CgbFlag::CgbBkwd,
            0b1100_0000 => CgbFlag::Cgb,
            0b1000_1000 | 0b1000_0100 => CgbFlag::Pgb,
            _ => CgbFlag::Dmg,
        }
    }

    /// If true, the cartridge supports SGB functions.
    pub fn sgb_flag(&self) -> bool {
        self.data[0x0146] == 0x03
    }

    /// Get the amount of ROM present on the cartridge (in bytes).
    pub fn rom_size(&self) -> u32 {
        match self.data[0x0148] {
            val if val <= 0x08 => (1 << val) * 32 * BYTES_IN_KIB,
            _ => 0,
        }
    }

    /// Get the amount of RAM present on the cartridge (in bytes).
    pub fn ram_size(&self) -> u32 {
        (match self.data[0x0149] {
            0x02 => 8,
            0x03 => 32,
            0x04 => 128,
            0x05 => 64,
            _ => 0,
        }) * BYTES_IN_KIB
    }

    /// If true, cartridge was intended to be sold in Japan. Otherwise, it was overseas only.
    pub fn jpn(&self) -> bool {
        self.data[0x014A] != 0x00
    }

    /// Get the version number of the game (usually 0).
    pub fn version_number(&self) -> u8 {
        self.data[0x014C]
    }

    /// Get the header checksum.
    pub fn checksum(&self) -> u8 {
        self.data[0x014D]
    }

    /// Get the global checksum.
    pub fn global_checksum(&self) -> u16 {
        ((self.data[0x014E] as u16) << 8) | (self.data[0x014F] as u16)
    }
}

/// All the different hardware features a cartridge can have.
#[derive(Debug, Clone)]
pub struct CartFeatures {
    pub rom_only: bool,
    pub rom: bool,
    pub mbc1: bool,
    pub mbc2: bool,
    pub mbc3: bool,
    pub mbc5: bool,
    pub mbc6: bool,
    pub mbc7: bool,
    pub ram: bool,
    pub battery: bool,
    pub mmm01: bool,
    pub timer: bool,
    pub rumble: bool,
    pub sensor: bool,
    pub pocket_camera: bool,
    pub bandai_tama5: bool,
    pub huc3: bool,
    pub huc1: bool,
}
impl CartFeatures {
    /// Read the [CartFeatures] from the cartridge header.
    fn from_data(data: &[u8; 0x10000]) -> Self {
        let mut cf = Self {
            rom_only: false,
            rom: false,
            mbc1: false,
            mbc2: false,
            mbc3: false,
            mbc5: false,
            mbc6: false,
            mbc7: false,
            ram: false,
            battery: false,
            mmm01: false,
            timer: false,
            rumble: false,
            sensor: false,
            pocket_camera: false,
            bandai_tama5: false,
            huc3: false,
            huc1: false,
        };
        match data[0x0147] {
            0x00 => cf.rom_only = true,
            0x01 => cf.mbc1 = true,
            0x02 => {
                cf.mbc1 = true;
                cf.ram = true;
            }
            0x03 => {
                cf.mbc1 = true;
                cf.ram = true;
                cf.battery = true;
            }
            0x05 => cf.mbc2 = true,
            0x06 => {
                cf.mbc2 = true;
                cf.battery = true;
            }
            0x08 => {
                cf.rom = true;
                cf.ram = true;
            }
            0x09 => {
                cf.rom = true;
                cf.ram = true;
                cf.battery = true;
            }
            0x0B => cf.mmm01 = true,
            0x0C => {
                cf.mmm01 = true;
                cf.ram = true;
            }
            0x0D => {
                cf.mmm01 = true;
                cf.ram = true;
                cf.battery = true;
            }
            0x0F => {
                cf.mbc3 = true;
                cf.timer = true;
                cf.battery = true;
            }
            0x10 => {
                cf.mbc3 = true;
                cf.timer = true;
                cf.ram = true;
                cf.battery = true;
            }
            0x11 => cf.mbc3 = true,
            0x12 => {
                cf.mbc3 = true;
                cf.ram = true;
            }
            0x13 => {
                cf.mbc3 = true;
                cf.ram = true;
                cf.battery = true;
            }
            0x19 => cf.mbc5 = true,
            0x1A => {
                cf.mbc5 = true;
                cf.ram = true;
            }
            0x1B => {
                cf.mbc5 = true;
                cf.ram = true;
                cf.battery = true;
            }
            0x1C => {
                cf.mbc5 = true;
                cf.rumble = true;
            }
            0x1D => {
                cf.mbc5 = true;
                cf.rumble = true;
                cf.ram = true;
            }
            0x1E => {
                cf.mbc5 = true;
                cf.rumble = true;
                cf.ram = true;
                cf.battery = true;
            }
            0x20 => cf.mbc6 = true,
            0x22 => {
                cf.mbc7 = true;
                cf.sensor = true;
                cf.rumble = true;
                cf.ram = true;
                cf.battery = true;
            }
            0xFC => cf.pocket_camera = true,
            0xFD => cf.bandai_tama5 = true,
            0xFE => cf.huc3 = true,
            0xFF => {
                cf.huc1 = true;
                cf.ram = true;
                cf.battery = true;
            }
            _ => {}
        };
        cf
    }
}

/// The different states of the CGB header flag.
pub enum CgbFlag {
    /// DMG cartridge. Compatible with DMG & CGB.
    Dmg,
    /// Cartridge with CGB enhancements that is backwards-compatible with DMG.
    CgbBkwd,
    /// Cartridge that works on CGB only.
    Cgb,
    /// Unknown PGB mode.
    Pgb,
}
