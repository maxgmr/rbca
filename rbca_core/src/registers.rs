//! All functionality related to the registers of the emulated CPU.
use std::default::Default;

/// The bit masks for the different flags of register `F`.
#[derive(Debug, Copy, Clone)]
pub enum RegFlag {
    /// `Z`: Zero flag
    Z = 0b1000_0000,
    /// `N`: Subtraction flag
    N = 0b0100_0000,
    /// `H`: Half Carry flag
    H = 0b0010_0000,
    /// `C`: Carry flag
    C = 0b0001_0000,
}

/// The registers of the emulated CPU.
#[derive(Debug, Default, Clone)]
pub struct Registers {
    /// Register `A` (accumulator)
    pub a: u8,
    /// Register `B`
    pub b: u8,
    /// Register `C`
    pub c: u8,
    /// Register `D`
    pub d: u8,
    /// Register `E`
    pub e: u8,
    f: u8,
    /// Register `H`
    pub h: u8,
    /// Register `L`
    pub l: u8,
}
impl Registers {
    /// Create new [Registers], all initialised to zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get register `AF`.
    pub fn get_af(&self) -> u16 {
        Self::get_virtual_register_helper(self.a, self.f)
    }

    /// Get register `BC`.
    pub fn get_bc(&self) -> u16 {
        Self::get_virtual_register_helper(self.b, self.c)
    }

    /// Get register `DE`.
    pub fn get_de(&self) -> u16 {
        Self::get_virtual_register_helper(self.d, self.e)
    }

    /// Get register `HL`.
    pub fn get_hl(&self) -> u16 {
        Self::get_virtual_register_helper(self.h, self.l)
    }

    /// Set register `AF`.
    pub fn set_af(&mut self, value: u16) {
        // Last nibble of `F` must always be zero.
        Self::set_virtual_register_helper(&mut self.a, &mut self.f, value & 0xFFF0)
    }

    /// Set register `BC`.
    pub fn set_bc(&mut self, value: u16) {
        Self::set_virtual_register_helper(&mut self.b, &mut self.c, value)
    }

    /// Set register `DE`.
    pub fn set_de(&mut self, value: u16) {
        Self::set_virtual_register_helper(&mut self.d, &mut self.e, value)
    }

    /// Set register `HL`.
    pub fn set_hl(&mut self, value: u16) {
        Self::set_virtual_register_helper(&mut self.h, &mut self.l, value)
    }

    /// Get flag.
    pub fn get_flag(&self, flag: RegFlag) -> bool {
        (self.f & (flag as u8)) != 0
    }

    /// Set flag.
    pub fn set_flag(&mut self, flag: RegFlag, value: bool) {
        let pos = (flag as u8).trailing_zeros();
        self.f = (self.f & !(0b1 << pos)) | ((if value { 1 } else { 0 }) << pos)
    }

    /// Abstraction of the common code for setting virtual 16-bit registers.
    fn set_virtual_register_helper(first_register: &mut u8, second_register: &mut u8, value: u16) {
        *first_register = ((value & 0xFF00) >> 8) as u8;
        *second_register = (value & 0x00FF) as u8;
    }

    /// Abstraction of the common code for getting virtual 16-bit registers.
    fn get_virtual_register_helper(first_register: u8, second_register: u8) -> u16 {
        ((first_register as u16) << 8) | (second_register as u16)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_virtual_registers() {
        let mut rs = Registers::default();
        assert_eq!(rs.get_af(), 0x0000);
        assert_eq!(rs.get_bc(), 0x0000);
        assert_eq!(rs.get_de(), 0x0000);
        assert_eq!(rs.get_hl(), 0x0000);

        rs.set_af(0x01A0);
        rs.set_bc(0x4567);
        rs.set_de(0x89AB);
        rs.set_hl(0xCDEF);
        assert_eq!(rs.get_af(), 0x01A0);
        assert_eq!(rs.get_bc(), 0x4567);
        assert_eq!(rs.get_de(), 0x89AB);
        assert_eq!(rs.get_hl(), 0xCDEF);
        assert_eq!(rs.a, 0x01);
        assert_eq!(rs.b, 0x45);
        assert_eq!(rs.c, 0x67);
        assert_eq!(rs.d, 0x89);
        assert_eq!(rs.e, 0xAB);
        assert_eq!(rs.f, 0xA0);
        assert_eq!(rs.h, 0xCD);
        assert_eq!(rs.l, 0xEF);

        rs.a = 0xFF;
        assert_eq!(rs.get_af(), 0xFFA0);
    }

    #[test]
    fn test_flags() {
        let mut rs = Registers::new();
        assert!(!rs.get_flag(RegFlag::Z));
        assert!(!rs.get_flag(RegFlag::N));
        assert!(!rs.get_flag(RegFlag::H));
        assert!(!rs.get_flag(RegFlag::C));
        assert_eq!(rs.f, 0x00);
        assert_eq!(rs.get_af(), 0x0000);

        rs.set_flag(RegFlag::C, true);
        assert!(rs.get_flag(RegFlag::C));
        assert_eq!(rs.f, 0b0001_0000);
        rs.set_flag(RegFlag::C, false);

        rs.set_flag(RegFlag::Z, true);
        assert!(rs.get_flag(RegFlag::Z));
        assert!(!rs.get_flag(RegFlag::N));
        assert_eq!(rs.get_af(), 0x0080);

        rs.set_flag(RegFlag::H, true);
        assert!(rs.get_flag(RegFlag::H));
        assert!(rs.get_flag(RegFlag::Z));
        assert_eq!(rs.get_af(), 0x00A0);

        rs.set_flag(RegFlag::Z, false);
        assert!(rs.get_flag(RegFlag::H));
        assert!(!rs.get_flag(RegFlag::Z));
        assert_eq!(rs.get_af(), 0x0020);

        rs.set_flag(RegFlag::N, true);
        rs.set_flag(RegFlag::C, true);
        assert!(!rs.get_flag(RegFlag::Z));
        assert!(rs.get_flag(RegFlag::N));
        assert!(rs.get_flag(RegFlag::H));
        assert!(rs.get_flag(RegFlag::C));
        assert_eq!(rs.get_af(), 0x0070);

        rs.set_flag(RegFlag::Z, true);
        assert_eq!(rs.get_af(), 0x00F0);
    }
}
