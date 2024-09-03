use std::default::Default;

/// Enum for accessing IE register components.
#[derive(Debug, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum IEHandler {
    /// Joypad interrupt handler
    JOYPAD = 0b0001_0000,
    /// Serial interrupt handler
    SERIAL = 0b0000_1000,
    /// Timer interrupt handler
    TIMER = 0b0000_0100,
    /// LCD interrupt handler
    LCD = 0b0000_0010,
    /// VBlank interrupt handler
    VBLANK = 0b0000_0001,
}

/// Interrupt Enable register.
#[derive(Debug, Clone)]
pub struct IERegister {
    byte: u8,
}
impl IERegister {
    /// Create a new [IERegister].
    pub fn new() -> Self {
        Self { byte: 0x00 }
    }

    /// Return the raw contents of the IE register. Not recommended for access.
    pub fn read_byte(&self) -> u8 {
        self.byte
    }

    /// Write the raw contents of the IE register. Use is strongly discouraged!
    pub fn write_byte(&mut self, byte: u8) {
        self.byte = byte
    }

    /// Get the value of the given interrupt handler.
    pub fn get(&self, handler: IEHandler) -> bool {
        (self.byte & (handler as u8)) != 0
    }

    /// Set the value of the given interrupt handler.
    pub fn set(&mut self, handler: IEHandler, value: bool) {
        let pos = (handler as u8).trailing_zeros();
        self.byte = (self.byte & !(0b1 << pos)) | ((if value { 1 } else { 0 }) << pos)
    }
}
impl Default for IERegister {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::{
        IEHandler::{JOYPAD, LCD, SERIAL, TIMER, VBLANK},
        *,
    };

    #[test]
    fn test_get_set() {
        let mut ie = IERegister::new();
        ie.write_byte(0b0001_0101);
        assert!(ie.get(JOYPAD));
        assert!(!ie.get(SERIAL));
        assert!(ie.get(TIMER));
        assert!(!ie.get(LCD));
        assert!(ie.get(VBLANK));

        ie.set(JOYPAD, false);
        assert_eq!(ie.read_byte(), 0b0000_0101);
        assert!(!ie.get(JOYPAD));

        ie.set(SERIAL, true);
        assert_eq!(ie.read_byte(), 0b0000_1101);
        assert!(ie.get(SERIAL));

        ie.set(TIMER, false);
        assert_eq!(ie.read_byte(), 0b0000_1001);
        assert!(!ie.get(TIMER));

        ie.set(LCD, true);
        assert_eq!(ie.read_byte(), 0b0000_1011);
        assert!(ie.get(SERIAL));

        ie.set(VBLANK, false);
        assert_eq!(ie.read_byte(), 0b0000_1010);
        assert!(!ie.get(VBLANK));
    }
}
