use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

/// For single-byte I/O registers whose bits act like flags.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Flags {
    byte: u8,
}
impl Flags {
    /// Create a new instance of the struct with its default boot values.
    pub fn new(default_vals: u8) -> Self {
        Self { byte: default_vals }
    }

    /// Read the contents of the register.
    pub fn read_byte(&self) -> u8 {
        self.byte
    }

    /// Get a mutable reference to the contents of the register.
    pub fn read_byte_mut(&mut self) -> &mut u8 {
        &mut self.byte
    }

    /// Overwrite the contents of the register.
    pub fn write_byte(&mut self, byte: u8) {
        self.byte = byte
    }

    /// Get the value of the given flag.
    pub fn get<F: FlagsEnum>(&self, flag: F) -> bool {
        (self.byte & flag.val()) != 0
    }

    /// Set the value of the given flag.
    pub fn set<F: FlagsEnum>(&mut self, flag: F, value: bool) {
        let pos = flag.val().trailing_zeros();
        self.byte = (self.byte & !(0b1 << pos)) | ((if value { 1 } else { 0 }) << pos)
    }
}
impl BitAnd for Flags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Flags::new(self.byte & rhs.byte)
    }
}
impl BitAndAssign for Flags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.byte &= rhs.byte;
    }
}
impl BitOr for Flags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Flags::new(self.byte | rhs.byte)
    }
}
impl BitOrAssign for Flags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.byte |= rhs.byte;
    }
}
impl BitXor for Flags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Flags::new(self.byte ^ rhs.byte)
    }
}
impl BitXorAssign for Flags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.byte ^= rhs.byte;
    }
}

/// For enums that index the bits of a [Flags].
pub trait FlagsEnum {
    /// Get the value of the enum.
    fn val(&self) -> u8;
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    enum TestFlagsEnum {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
    }
    impl FlagsEnum for TestFlagsEnum {
        fn val(&self) -> u8 {
            match self {
                Self::H => 0b0000_0001,
                Self::G => 0b0000_0010,
                Self::F => 0b0000_0100,
                Self::E => 0b0000_1000,
                Self::D => 0b0001_0000,
                Self::C => 0b0010_0000,
                Self::B => 0b0100_0000,
                Self::A => 0b1000_0000,
            }
        }
    }

    #[test]
    fn test_get_set() {
        use TestFlagsEnum::*;

        let mut flags = Flags::new(0b0000_0000);
        assert_eq!(flags.read_byte(), 0b0000_0000);
        flags.write_byte(0b0101_0101);
        assert_eq!(flags.read_byte(), 0b0101_0101);
        assert!(!flags.get(A));
        assert!(flags.get(B));
        assert!(!flags.get(C));
        assert!(flags.get(D));
        assert!(!flags.get(E));
        assert!(flags.get(F));
        assert!(!flags.get(G));
        assert!(flags.get(H));

        flags.set(A, true);
        assert_eq!(flags.read_byte(), 0b1101_0101);
        assert!(flags.get(A));

        flags.set(B, false);
        assert_eq!(flags.read_byte(), 0b1001_0101);
        assert!(!flags.get(B));

        flags.set(C, false);
        assert_eq!(flags.read_byte(), 0b1001_0101);
        assert!(!flags.get(C));

        flags.set(D, true);
        assert_eq!(flags.read_byte(), 0b1001_0101);
        assert!(flags.get(D));

        flags.set(E, true);
        assert_eq!(flags.read_byte(), 0b1001_1101);
        assert!(flags.get(E));

        flags.set(F, false);
        assert_eq!(flags.read_byte(), 0b1001_1001);
        assert!(!flags.get(F));

        flags.set(G, true);
        assert_eq!(flags.read_byte(), 0b1001_1011);
        assert!(flags.get(G));

        flags.set(H, false);
        assert_eq!(flags.read_byte(), 0b1001_1010);
        assert!(!flags.get(H));
    }

    #[test]
    fn test_bitwise() {
        let mut f1 = Flags::new(0b0101_0000);
        let f2 = Flags::new(0b0110_1010);

        assert_eq!((f1 | f2).read_byte(), 0b0111_1010);
        assert_eq!((f1 & f2).read_byte(), 0b0100_0000);
        assert_eq!((f1 ^ f2).read_byte(), 0b0011_1010);

        f1 |= f2;
        assert_eq!(f1.read_byte(), 0b0111_1010);
        f1 &= f2;
        assert_eq!(f1.read_byte(), 0b0110_1010);
        let mut f1 = Flags::new(0b1110_0110);
        f1 ^= f2;
        assert_eq!(f1.read_byte(), 0b1000_1100);
    }
}
