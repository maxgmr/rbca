//! All functionality related to the registers of the CPU.
use std::default::Default;

pub const ZERO_FLAG_BYTE_POS: usize = 7;
pub const ADD_SUB_FLAG_BYTE_POS: usize = 6;
pub const HALF_CARRY_FLAG_BYTE_POS: usize = 5;
pub const CARRY_FLAG_BYTE_POS: usize = 4;

/// All the CPU registers.
#[derive(Debug, Default)]
pub struct Registers {
    /// 8-bit register `a`.
    /// First constituent of virtual 16-bit register `af`.
    pub a: u8,
    /// 8-bit register `b`.
    /// First constituent of virtual 16-bit register `bc`.
    pub b: u8,
    /// 8-bit register `c`.
    /// Second constituent of virtual 16-bit register `bc`.
    pub c: u8,
    /// 8-bit register `d`.
    /// First constituent of virtual 16-bit register `de`.
    pub d: u8,
    /// 8-bit register `e`.
    /// Second constituent of virtual 16-bit register `de`.
    pub e: u8,
    /// 8-bit register `f`, the "flag" register.
    /// Bit     Name        Set     Clr     Expl.
    /// 7       zf          Z       NZ      Zero Flag
    /// 6       n           -       -       Add/Sub-Flag (BCD)
    /// 5       h           -       -       Half Carry Flag (BCD)
    /// 4       cy          C       NC      Carry Flag
    /// 3-0     -           -       -       Not used (always zero)
    /// Second constituent of virtual 16-bit register `af`.
    pub f: u8,
    /// 8-bit register `h`.
    /// First constituent of virtual 16-bit register `hl`.
    pub h: u8,
    /// 8-bit register `l`.
    /// Second constituent of virtual 16-bit register `hl`.
    pub l: u8,
}
impl Registers {
    /// Create new [Registers] all initialised to zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the virtual 16-bit register `af`.
    pub fn get_af(&self) -> u16 {
        Self::get_virtual_register_helper(self.a, self.f)
    }

    /// Set the virtual 16-bit register `af`.
    pub fn set_af(&mut self, value: u16) {
        Self::set_virtual_register_helper(&mut self.a, &mut self.f, value);
    }

    /// Return the virtual 16-bit register `bc`.
    pub fn get_bc(&self) -> u16 {
        Self::get_virtual_register_helper(self.b, self.c)
    }

    /// Set the virtual 16-bit register `bc`.
    pub fn set_bc(&mut self, value: u16) {
        Self::set_virtual_register_helper(&mut self.b, &mut self.c, value);
    }

    /// Return the virtual 16-bit register `de`.
    pub fn get_de(&self) -> u16 {
        Self::get_virtual_register_helper(self.d, self.e)
    }

    /// Set the virtual 16-bit register `de`.
    pub fn set_de(&mut self, value: u16) {
        Self::set_virtual_register_helper(&mut self.d, &mut self.e, value);
    }

    /// Return the virtual 16-bit register `hl`.
    pub fn get_hl(&self) -> u16 {
        Self::get_virtual_register_helper(self.h, self.l)
    }

    /// Set the virtual 16-bit register `hl`.
    pub fn set_hl(&mut self, value: u16) {
        Self::set_virtual_register_helper(&mut self.h, &mut self.l, value);
    }

    /// Return the zero flag.
    pub fn get_zero_flag(&self) -> bool {
        (self.f >> ZERO_FLAG_BYTE_POS) != 0
    }

    /// Set the zero flag.
    pub fn set_zero_flag(&mut self, value: bool) {
        self.set_flag_helper(ZERO_FLAG_BYTE_POS, value);
    }

    /// Return the add/sub-flag.
    pub fn get_add_sub_flag(&self) -> bool {
        ((self.f & 0b_0100_0000) >> ADD_SUB_FLAG_BYTE_POS) != 0
    }

    /// Set the add/sub-flag.
    pub fn set_add_sub_flag(&mut self, value: bool) {
        self.set_flag_helper(ADD_SUB_FLAG_BYTE_POS, value);
    }

    /// Return the half carry flag.
    pub fn get_half_carry_flag(&self) -> bool {
        ((self.f & 0b_0010_0000) >> HALF_CARRY_FLAG_BYTE_POS) != 0
    }

    /// Set the half carry flag.
    pub fn set_half_carry_flag(&mut self, value: bool) {
        self.set_flag_helper(HALF_CARRY_FLAG_BYTE_POS, value);
    }

    /// Return the carry flag.
    pub fn get_carry_flag(&self) -> bool {
        ((self.f & 0b_0001_0000) >> CARRY_FLAG_BYTE_POS) != 0
    }

    /// Set the carry flag.
    pub fn set_carry_flag(&mut self, value: bool) {
        self.set_flag_helper(CARRY_FLAG_BYTE_POS, value);
    }

    /// Abstraction of the common code for getting virtual 16-bit registers.
    fn get_virtual_register_helper(first_register: u8, second_register: u8) -> u16 {
        ((first_register as u16) << 8) | (second_register as u16)
    }

    /// Abstraction of the common code for getting virtual 16-bit registers.
    fn set_virtual_register_helper(first_register: &mut u8, second_register: &mut u8, value: u16) {
        *first_register = ((value & 0xFF00) >> 8) as u8;
        *second_register = (value & 0x00FF) as u8;
    }

    /// Abstraction: sets the nth bit of the `f` register to the given value.
    fn set_flag_helper(&mut self, n: usize, value: bool) {
        self.f = (self.f & !(0b1 << n)) | ((if value { 1 } else { 0 }) << n)
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
    fn test_carry_flags() {
        let mut rs = Registers::new();
        assert!(!rs.get_zero_flag());
        assert!(!rs.get_add_sub_flag());
        assert!(!rs.get_half_carry_flag());
        assert!(!rs.get_carry_flag());
        assert_eq!(rs.f, 0x00);
        assert_eq!(rs.get_af(), 0x0000);

        rs.set_zero_flag(true);
        assert!(rs.get_zero_flag());
        assert!(!rs.get_add_sub_flag());
        assert_eq!(rs.get_af(), 0x0080);

        rs.set_zero_flag(true);
        assert!(rs.get_zero_flag());
        assert!(!rs.get_half_carry_flag());
        assert_eq!(rs.get_af(), 0x0080);

        rs.set_half_carry_flag(true);
        assert!(rs.get_half_carry_flag());
        assert!(rs.get_zero_flag());
        assert_eq!(rs.get_af(), 0x00A0);

        rs.set_zero_flag(false);
        assert!(rs.get_half_carry_flag());
        assert!(!rs.get_zero_flag());
        assert_eq!(rs.get_af(), 0x0020);

        rs.set_add_sub_flag(true);
        rs.set_carry_flag(true);
        assert!(!rs.get_zero_flag());
        assert!(rs.get_add_sub_flag());
        assert!(rs.get_half_carry_flag());
        assert!(rs.get_carry_flag());
        assert_eq!(rs.get_af(), 0x0070);

        rs.set_zero_flag(true);
        assert_eq!(rs.get_af(), 0x00F0);
    }
}
