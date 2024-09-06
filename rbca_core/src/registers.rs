//! All functionality related to the registers of the emulated CPU.
use std::{default::Default, fmt::Display};

use strum_macros::EnumIter;

use crate::{Flags, FlagsEnum};

/// Enum to define the register target of a function.
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum Target {
    /// Register `A` (accumulator)
    A,
    /// Register `B`
    B,
    /// Register `C`
    C,
    /// Register `D`
    D,
    /// Register `E`
    E,
    /// Register `H`
    H,
    /// Register `L`
    L,
}
impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::A => "A",
                Self::B => "B",
                Self::C => "C",
                Self::D => "D",
                Self::E => "E",
                Self::H => "H",
                Self::L => "L",
            }
        )
    }
}

/// Enum to define the virtual register target of a function.
#[derive(Debug, Copy, Clone, EnumIter, PartialEq)]
pub enum VirtTarget {
    /// Virtual register `AF`
    AF,
    /// Virtual register `BC`
    BC,
    /// Virtual register `DE`
    DE,
    /// Virtual register `HL`
    HL,
}
impl Display for VirtTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AF => "AF",
                Self::BC => "BC",
                Self::DE => "DE",
                Self::HL => "HL",
            }
        )
    }
}

/// The bit masks for the different flags of register `F`.
#[derive(Debug, Copy, Clone)]
pub enum RegFlag {
    /// `Z`: Zero flag
    Z,
    /// `N`: Subtraction flag
    N,
    /// `H`: Half Carry flag
    H,
    /// `C`: Carry flag
    C,
}
impl FlagsEnum for RegFlag {
    fn val(&self) -> u8 {
        match self {
            Self::Z => 0b1000_0000,
            Self::N => 0b0100_0000,
            Self::H => 0b0010_0000,
            Self::C => 0b0001_0000,
        }
    }
}
impl Display for RegFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Z => "Z",
                Self::N => "N",
                Self::H => "H",
                Self::C => "C",
            }
        )
    }
}

/// The registers of the emulated CPU.
#[derive(Debug, Clone, PartialEq)]
pub struct Registers {
    /// Register `A` (accumulator)
    a: u8,
    /// Register `B`
    b: u8,
    /// Register `C`
    c: u8,
    /// Register `D`
    d: u8,
    /// Register `E`
    e: u8,
    f: Flags,
    /// Register `H`
    h: u8,
    /// Register `L`
    l: u8,
}
impl Registers {
    /// Create new [Registers], all initialised to zero.
    pub fn new() -> Self {
        Self {
            // a: 0x01,
            a: 0x00,
            // b: 0x00,
            b: 0x00,
            // c: 0x13,
            c: 0x00,
            // d: 0x00,
            d: 0x00,
            // e: 0xD8,
            e: 0x00,
            // f: Flags::new(RegFlag::Z.val() | RegFlag::H.val() | RegFlag::C.val()),
            f: Flags::new(0x00),
            // h: 0x01,
            h: 0x00,
            // l: 0x4D,
            l: 0x00,
        }
    }

    /// Get the value of a register.
    pub fn get_reg(&self, target: Target) -> u8 {
        match target {
            Target::A => self.a,
            Target::B => self.b,
            Target::C => self.c,
            Target::D => self.d,
            Target::E => self.e,
            Target::H => self.h,
            Target::L => self.l,
        }
    }

    /// Set the value of a register.
    pub fn set_reg(&mut self, target: Target, value: u8) {
        let register = match target {
            Target::A => &mut self.a,
            Target::B => &mut self.b,
            Target::C => &mut self.c,
            Target::D => &mut self.d,
            Target::E => &mut self.e,
            Target::H => &mut self.h,
            Target::L => &mut self.l,
        };
        *register = value;
    }

    /// Get the value of a given virtual register.
    pub fn get_virt_reg(&self, target: VirtTarget) -> u16 {
        let (first_register, second_register) = match target {
            VirtTarget::AF => (self.a, self.f.read_byte()),
            VirtTarget::BC => (self.b, self.c),
            VirtTarget::DE => (self.d, self.e),
            VirtTarget::HL => (self.h, self.l),
        };
        ((first_register as u16) << 8) | (second_register as u16)
    }

    /// Set the value of a given virtual register.
    pub fn set_virt_reg(&mut self, target: VirtTarget, value: u16) {
        let (first_register, second_register) = match target {
            VirtTarget::AF => (&mut self.a, self.f.read_byte_mut()),
            VirtTarget::BC => (&mut self.b, &mut self.c),
            VirtTarget::DE => (&mut self.d, &mut self.e),
            VirtTarget::HL => (&mut self.h, &mut self.l),
        };
        *first_register = ((value & 0xFF00) >> 8) as u8;
        if target != VirtTarget::AF {
            *second_register = (value & 0x00FF) as u8;
        }
    }

    /// Get flag.
    pub fn get_flag(&self, flag: RegFlag) -> bool {
        self.f.get(flag)
    }

    /// Set flag.
    pub fn set_flag(&mut self, flag: RegFlag, value: bool) {
        self.f.set(flag, value)
    }

    /// Reset flags.
    pub fn reset_flags(&mut self) {
        self.f.write_byte(0b0000_0000);
    }

    /// Get print-friendly regs string.
    pub fn regs_string(&self) -> String {
        format!(
            "A: {:#04X}, B: {:#04X}, C: {:#04X}, D: {:#04X}, E: {:#04X}, H: {:#04X}, L: {:#04X}",
            &self.get_reg(Target::A),
            &self.get_reg(Target::B),
            &self.get_reg(Target::C),
            &self.get_reg(Target::D),
            &self.get_reg(Target::E),
            &self.get_reg(Target::H),
            &self.get_reg(Target::L),
        )
    }

    /// Get print-friendly flags string.
    pub fn flags_string(&self) -> String {
        format!(
            "Z: {}, N: {}, H: {}, C: {}",
            &self.get_flag(RegFlag::Z),
            &self.get_flag(RegFlag::N),
            &self.get_flag(RegFlag::H),
            &self.get_flag(RegFlag::C)
        )
    }
}
impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_registers() {
        let mut rs = Registers::new();

        rs.set_reg(Target::A, 0x12);
        rs.set_reg(Target::B, 0x34);
        rs.set_reg(Target::C, 0x56);
        rs.set_reg(Target::D, 0x78);
        rs.set_reg(Target::E, 0x9A);
        rs.set_reg(Target::H, 0xBC);
        rs.set_reg(Target::L, 0xDE);
        assert_eq!(rs.get_reg(Target::A), 0x12);
        assert_eq!(rs.get_reg(Target::B), 0x34);
        assert_eq!(rs.get_reg(Target::C), 0x56);
        assert_eq!(rs.get_reg(Target::D), 0x78);
        assert_eq!(rs.get_reg(Target::E), 0x9A);
        assert_eq!(rs.get_reg(Target::H), 0xBC);
        assert_eq!(rs.get_reg(Target::L), 0xDE);

        rs.set_reg(Target::A, rs.get_reg(Target::H));
        assert_eq!(rs.get_reg(Target::A), 0xBC);
        assert_eq!(rs.get_reg(Target::H), 0xBC);
    }

    #[test]
    fn test_virtual_registers() {
        let mut rs = Registers::default();

        rs.set_virt_reg(VirtTarget::AF, 0x01A0);
        rs.set_virt_reg(VirtTarget::BC, 0x4567);
        rs.set_virt_reg(VirtTarget::DE, 0x89AB);
        rs.set_virt_reg(VirtTarget::HL, 0xCDEF);
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x0100);
        assert_eq!(rs.get_virt_reg(VirtTarget::BC), 0x4567);
        assert_eq!(rs.get_virt_reg(VirtTarget::DE), 0x89AB);
        assert_eq!(rs.get_virt_reg(VirtTarget::HL), 0xCDEF);
        assert_eq!(rs.a, 0x01);
        assert_eq!(rs.b, 0x45);
        assert_eq!(rs.c, 0x67);
        assert_eq!(rs.d, 0x89);
        assert_eq!(rs.e, 0xAB);
        assert_eq!(rs.f.read_byte(), 0x00);
        assert_eq!(rs.h, 0xCD);
        assert_eq!(rs.l, 0xEF);

        rs.a = 0xFF;
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0xFF00);
    }

    #[test]
    fn test_flags() {
        let mut rs = Registers::new();
        assert!(!rs.get_flag(RegFlag::Z));
        assert!(!rs.get_flag(RegFlag::N));
        assert!(!rs.get_flag(RegFlag::H));
        assert!(!rs.get_flag(RegFlag::C));
        assert_eq!(rs.f.read_byte(), 0x00);
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x0000);

        rs.set_flag(RegFlag::C, true);
        assert!(rs.get_flag(RegFlag::C));
        assert_eq!(rs.f.read_byte(), 0b0001_0000);
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

        rs.reset_flags();
        assert!(!rs.get_flag(RegFlag::Z));
        assert!(!rs.get_flag(RegFlag::N));
        assert!(!rs.get_flag(RegFlag::H));
        assert!(!rs.get_flag(RegFlag::C));
        assert_eq!(rs.get_virt_reg(VirtTarget::AF), 0x0000);
    }
}
