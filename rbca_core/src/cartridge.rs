//! Functionality related to Game Boy cartridges.
use std::{fmt::Debug, fmt::Display, fs::File, io::Read};

use camino::Utf8Path;

mod empty;
mod mbc1;
mod rom_only;

// Re-exports
pub use empty::CartEmpty;

const BOOT_ROM_PATH: &str = "../dmg-boot.bin";

const BYTES_IN_KIB: u32 = 1024;

const LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

/// Load a cartridge.
pub fn load_cartridge<P: AsRef<Utf8Path>>(filepath: P) -> Box<dyn Cartridge> {
    let mut file_buf = vec![];
    if let Err(e) = File::open(filepath.as_ref()).and_then(|mut f| f.read_to_end(&mut file_buf)) {
        panic!("{e}");
    }

    let cart_features = CartFeatures::from_data(&file_buf);
    if cart_features.mbc1 {
        Box::new(mbc1::CartMBC1::new(file_buf))
    } else if cart_features.rom_only {
        Box::new(rom_only::CartRomOnly::new(file_buf, cart_features))
    } else {
        unimplemented!("Unimplemented cartridge type: {cart_features}");
    }
}

/// A Game Boy cartridge.
pub trait Cartridge: Debug {
    /// Get the full raw ROM contents of the cartridge.
    fn rom(&self) -> &[u8];

    /// Get the cartridge features.
    fn cart_features(&self) -> &CartFeatures;

    /// Read a byte from the cartridge ROM.
    ///
    /// Default: Read directly from address without any banking.
    fn read_rom(&self, address: u16) -> u8 {
        self.rom()[address as usize]
    }

    /// Read a byte from the cartridge RAM.
    ///
    /// Default: no RAM, return garbage value
    #[allow(unused_variables)]
    fn read_ram(&self, address: u16) -> u8 {
        0xFF
    }

    /// "Write" a byte to the cartridge ROM. This usually has some other effect, such as setting an
    /// internal cartridge register.
    ///
    /// Default value: Nothing happens on write.
    #[allow(unused_variables)]
    fn write_rom(&mut self, address: u16, value: u8) {}

    /// Write a byte to the cartridge RAM.
    ///
    /// Default: no RAM, nothing happens.
    #[allow(unused_variables)]
    fn write_ram(&mut self, address: u16, value: u8) {}

    /// Get the logo shown at startup.
    fn logo(&self) -> &[u8] {
        &self.rom()[0x0104..=0x0133]
    }

    /// Verify the accuracy of the logo. Return true iff logo is okay.
    fn validate_logo(&self) -> bool {
        for (i, logo_byte) in LOGO.iter().enumerate() {
            if self.rom()[0x0104_usize + i] != *logo_byte {
                return false;
            }
        }
        true
    }

    /// Get the cartridge title.
    fn title(&self) -> &str {
        self.title_helper(0x0143)
    }
    /// Get the 11-byte cartridge title (for CGB & later cartridges).
    fn title_cgb(&self) -> &str {
        self.title_helper(0x013E)
    }
    /// Helper for title functions.
    fn title_helper(&self, end_index: u16) -> &str {
        match std::str::from_utf8(&self.rom()[0x0134..=(end_index as usize)]) {
            Ok(val) => val,
            _ => std::str::from_utf8(&self.rom()[0x0134..=(end_index as usize - 1)])
                .unwrap_or_default(),
        }
    }

    /// Get the manufacturer code.
    fn manufacturer_code(&self) -> &[u8] {
        &self.rom()[0x13F..=0x0142]
    }

    /// Get the CGB flag.
    fn cgb_flag(&self) -> CgbFlag {
        match self.rom()[0x0143] {
            0b1000_0000 => CgbFlag::CgbBkwd,
            0b1100_0000 => CgbFlag::Cgb,
            0b1000_1000 | 0b1000_0100 => CgbFlag::Pgb,
            _ => CgbFlag::Dmg,
        }
    }

    /// Return true iff the cartridge supports SGB functionality.
    fn sgb_flag(&self) -> bool {
        self.rom()[0x0146] == 0x03
    }

    /// Get the amount of ROM present on the cartridge (in bytes).
    fn rom_size(&self) -> u32 {
        match self.rom()[0x0148] {
            val if val <= 0x08 => (1 << val) * 32 * BYTES_IN_KIB,
            _ => 0,
        }
    }

    /// Get the amount of RAM present on the cartridge (in bytes).
    fn ram_size(&self) -> u32 {
        (match self.rom()[0x0149] {
            0x02 => 8,
            0x03 => 32,
            0x04 => 128,
            0x05 => 64,
            _ => 0,
        }) * BYTES_IN_KIB
    }

    /// Return true iff cartridge was intended to be sold in Japan. Otherwise, it was overseas
    /// only.
    fn jpn(&self) -> bool {
        self.rom()[0x014A] != 0x00
    }

    /// Get the version number of the cartridge (usually 0).
    fn version_number(&self) -> u8 {
        self.rom()[0x014C]
    }

    /// Get the header checksum.
    fn checksum(&self) -> u8 {
        self.rom()[0x014D]
    }

    /// Validate checksum. Return None if checksum passes. Otherwise, return the incorrect checksum
    /// that was produced.
    fn validate_checksum(&self) -> Option<u8> {
        let checksum = checksum_fn(self.rom());
        if self.checksum() == checksum {
            // Checksum OK!
            None
        } else {
            // Bad checksum!
            Some(checksum)
        }
    }

    /// Get the global checksum.
    fn global_checksum(&self) -> u16 {
        ((self.rom()[0x014E] as u16) << 8) | (self.rom()[0x014F] as u16)
    }

    /// Get some header info formatted as a nice String.
    fn header_info(&self) -> String {
        format!(
            "Cart Info
            \tTitle         {}
            \tType          {}
            \tROM Size      {} KiB ({:#X} bytes)
            \tRAM Size      {} KiB ({:#X} bytes)
            \tChecksum      {}
            \tLogo          {}",
            self.title(),
            self.cart_features(),
            self.rom_size() / BYTES_IN_KIB,
            self.rom_size(),
            self.ram_size() / BYTES_IN_KIB,
            self.ram_size(),
            if let Some(val) = self.validate_checksum() {
                format!("Failed! ({:#04X} != {:#04X})", self.checksum(), val)
            } else {
                String::from("OK!")
            },
            if self.validate_logo() {
                String::from("OK!")
            } else {
                String::from("Invalid!")
            },
        )
    }
}

/// The function used to calculate the header checksum.
pub fn checksum_fn(rom: &[u8]) -> u8 {
    let mut checksum: u8 = 0;
    #[allow(clippy::needless_range_loop)]
    for i in 0x0134..=0x014C {
        checksum = checksum.wrapping_sub(rom[i]).wrapping_sub(1);
    }
    checksum
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

/// All the different hardware features a cartridge can have.
#[derive(Debug, Default, Clone)]
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
    fn from_data(data: &[u8]) -> Self {
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
impl Display for CartFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str_vec: Vec<&'static str> = vec![];
        if self.rom_only {
            str_vec.push("ROM ONLY");
        }
        if self.rom {
            str_vec.push("ROM");
        }
        if self.mbc1 {
            str_vec.push("MBC1");
        }
        if self.mbc2 {
            str_vec.push("MBC2");
        }
        if self.mbc3 {
            str_vec.push("MBC3");
        }
        if self.mbc5 {
            str_vec.push("MBC5");
        }
        if self.mbc6 {
            str_vec.push("MBC6");
        }
        if self.mbc7 {
            str_vec.push("MBC7");
        }
        if self.ram {
            str_vec.push("RAM");
        }
        if self.battery {
            str_vec.push("BATTERY");
        }
        if self.mmm01 {
            str_vec.push("MMM01");
        }
        if self.timer {
            str_vec.push("TIMER");
        }
        if self.rumble {
            str_vec.push("RUMBLE");
        }
        if self.sensor {
            str_vec.push("SENSOR");
        }
        if self.pocket_camera {
            str_vec.push("POCKET CAMERA");
        }
        if self.bandai_tama5 {
            str_vec.push("BANDAI TAMA5");
        }
        if self.huc3 {
            str_vec.push("HuC3");
        }
        if self.huc1 {
            str_vec.push("HuC1");
        }
        write!(f, "{}", str_vec.join("+"))
    }
}
