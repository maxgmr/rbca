//! All functionality related to the registers of the emulated CPU.
use std::default::Default;

/// Enum to define the register target of a function.
#[derive(Debug, Copy, Clone)]
pub enum Target {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

/// Enum to define the virtual register target of a function.
#[derive(Debug, Copy, Clone)]
pub enum VirtTarget {
    AF,
    BC,
    DE,
    HL,
}

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

    /// Get the value of a given virtual register.
    pub fn get_virt_reg(&self, target: VirtTarget) -> u16 {
        let (first_register, second_register) = match target {
            VirtTarget::AF => (self.a, self.f),
            VirtTarget::BC => (self.a, self.f),
            VirtTarget::DE => (self.d, self.e),
            VirtTarget::HL => (self.h, self.l),
        };
        ((first_register as u16) << 8) | (second_register as u16)
    }

    /// Set the value of a given virtual register.
    pub fn set_virt_reg(&mut self, target: VirtTarget, value: u16) {
        let (first_register, second_register) = match target {
            VirtTarget::AF => (&mut self.a, &mut self.f),
            VirtTarget::BC => (&mut self.a, &mut self.f),
            VirtTarget::DE => (&mut self.d, &mut self.e),
            VirtTarget::HL => (&mut self.h, &mut self.l),
        };
        *first_register = ((value & 0xFF00) >> 8) as u8;
        *second_register = (value & 0x00FF) as u8;
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
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_virtual_registers() {
        let mut rs = Registers::default();
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x0000);
        assert_eq!(rs.get_virt_reg(VirtTarget::BC), 0x0000);
        assert_eq!(rs.get_virt_reg(VirtTarget::DE), 0x0000);
        assert_eq!(rs.get_virt_reg(VirtTarget::HL), 0x0000);

        rs.set_virt_reg(VirtTarget::AF, 0x01A0);
        rs.set_virt_reg(VirtTarget::BC, 0x4567);
        rs.set_virt_reg(VirtTarget::DE, 0x89AB);
        rs.set_virt_reg(VirtTarget::HL, 0xCDEF);
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x01A0);
        assert_eq!(rs.get_virt_reg(VirtTarget::BC), 0x4567);
        assert_eq!(rs.get_virt_reg(VirtTarget::DE), 0x89AB);
        assert_eq!(rs.get_virt_reg(VirtTarget::HL), 0xCDEF);
        assert_eq!(rs.a, 0x01);
        assert_eq!(rs.b, 0x45);
        assert_eq!(rs.c, 0x67);
        assert_eq!(rs.d, 0x89);
        assert_eq!(rs.e, 0xAB);
        assert_eq!(rs.f, 0xA0);
        assert_eq!(rs.h, 0xCD);
        assert_eq!(rs.l, 0xEF);

        rs.a = 0xFF;
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0xFFA0);
    }

    #[test]
    fn test_flags() {
        let mut rs = Registers::new();
        assert!(!rs.get_flag(RegFlag::Z));
        assert!(!rs.get_flag(RegFlag::N));
        assert!(!rs.get_flag(RegFlag::H));
        assert!(!rs.get_flag(RegFlag::C));
        assert_eq!(rs.f, 0x00);
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x0000);

        rs.set_flag(RegFlag::C, true);
        assert!(rs.get_flag(RegFlag::C));
        assert_eq!(rs.f, 0b0001_0000);
        rs.set_flag(RegFlag::C, false);

        rs.set_flag(RegFlag::Z, true);
        assert!(rs.get_flag(RegFlag::Z));
        assert!(!rs.get_flag(RegFlag::N));
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x0080);

        rs.set_flag(RegFlag::H, true);
        assert!(rs.get_flag(RegFlag::H));
        assert!(rs.get_flag(RegFlag::Z));
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x00A0);

        rs.set_flag(RegFlag::Z, false);
        assert!(rs.get_flag(RegFlag::H));
        assert!(!rs.get_flag(RegFlag::Z));
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x0020);

        rs.set_flag(RegFlag::N, true);
        rs.set_flag(RegFlag::C, true);
        assert!(!rs.get_flag(RegFlag::Z));
        assert!(rs.get_flag(RegFlag::N));
        assert!(rs.get_flag(RegFlag::H));
        assert!(rs.get_flag(RegFlag::C));
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x0070);

        rs.set_flag(RegFlag::Z, true);
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x00F0);
    }
}
