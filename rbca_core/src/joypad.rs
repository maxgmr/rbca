use std::default::Default;

use crate::{Flags, FlagsEnum};

use Button::{Down, Left, Right, Select, Start, Up, A, B};
use Joyp::{ARight, BLeft, SelectButtons, SelectDPad, SelectUp, StartDown};

/// Device joypad (buttons).
#[derive(Debug)]
pub struct Joypad {
    /// Used to alert MMU that the joypad triggered some interrupt flags.
    pub interrupt_flags: Flags,
    data: Flags,
    internal_buttons: Flags,
    internal_dpad: Flags,
}
impl Joypad {
    /// Create a new [Joypad].
    pub fn new() -> Self {
        Self {
            interrupt_flags: Flags::new(0b0000_0000),
            data: Flags::new(0b0011_1111),
            internal_buttons: Flags::new(0b0011_1111),
            internal_dpad: Flags::new(0b0011_1111),
        }
    }

    /// Directly read the byte at the given address.
    pub fn read_byte(&self) -> u8 {
        self.data.read_byte()
    }

    /// Directly write to the byte at the given address.
    pub fn write_byte(&mut self, value: u8) {
        let value_flags = Flags::new(value);
        self.data.set(SelectButtons, value_flags.get(SelectButtons));
        self.data.set(SelectDPad, value_flags.get(SelectDPad));
        self.update();
    }

    /// Update Joypad state to match the pressed buttons.
    fn update(&mut self) {
        let old_vals = self.data.read_byte() & 0b0000_1111;
        let mut new_vals = Flags::new(0b0000_1111);

        if !self.data.get(SelectButtons) {
            // Add any pressed buttons to current data
            new_vals &= self.internal_buttons;
        }

        if !self.data.get(SelectDPad) {
            // Add any pressed D-pad directions to current data
            new_vals &= self.internal_dpad;
        }

        // If no buttons were pressed, but now some are...
        if old_vals == 0b0000_1111 && new_vals != Flags::new(0b0000_1111) {
            // ...activate interrupt flag.
            self.interrupt_flags |= Flags::new(0b0001_0000);
        }

        println!("{:#010b}, {:#010b}", old_vals, new_vals.read_byte());

        // Update buttons pressed, but not the "Select" state
        self.data = (self.data & Flags::new(0b1111_0000)) | new_vals;
    }

    /// Handle a pressed button.
    pub fn button_down(&mut self, button: Button) {
        match button {
            Up => self.internal_dpad.set(SelectUp, false),
            Down => self.internal_dpad.set(StartDown, false),
            Left => self.internal_dpad.set(BLeft, false),
            Right => self.internal_dpad.set(ARight, false),
            A => self.internal_buttons.set(ARight, false),
            B => self.internal_buttons.set(BLeft, false),
            Start => self.internal_buttons.set(StartDown, false),
            Select => self.internal_buttons.set(SelectUp, false),
        }
        self.update();
    }

    /// Handle a released button.
    pub fn button_up(&mut self, button: Button) {
        match button {
            Up => self.internal_dpad.set(SelectUp, true),
            Down => self.internal_dpad.set(StartDown, true),
            Left => self.internal_dpad.set(BLeft, true),
            Right => self.internal_dpad.set(ARight, true),
            A => self.internal_buttons.set(ARight, true),
            B => self.internal_buttons.set(BLeft, true),
            Start => self.internal_buttons.set(StartDown, true),
            Select => self.internal_buttons.set(SelectUp, true),
        }
        self.update();
    }
}
impl Default for Joypad {
    fn default() -> Self {
        Self::new()
    }
}

/// Button enum.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

/// Joypad enum.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Joyp {
    /// If 0, buttons (SsBA) can be read from lower nibble.
    SelectButtons,
    /// If 0, directional keys can be read from lower nibble.
    SelectDPad,
    /// 0 = pressed.
    StartDown,
    /// 0 = pressed.
    SelectUp,
    /// 0 = pressed.
    BLeft,
    /// 0 = pressed.
    ARight,
}
impl FlagsEnum for Joyp {
    fn val(&self) -> u8 {
        match self {
            Self::SelectButtons => 0b0010_0000,
            Self::SelectDPad => 0b0001_0000,
            Self::StartDown => 0b0000_1000,
            Self::SelectUp => 0b0000_0100,
            Self::BLeft => 0b0000_0010,
            Self::ARight => 0b0000_0001,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_joypad() {
        let mut joypad = Joypad::new();
        assert_eq!(joypad.read_byte(), 0b0011_1111);
        assert_eq!(joypad.interrupt_flags.read_byte(), 0b0000_0000);

        // Enable reading from buttons
        joypad.write_byte(0b0001_0000);
        assert_eq!(joypad.read_byte(), 0b0001_1111);

        // Press A button
        joypad.button_down(A);
        assert_eq!(joypad.interrupt_flags.read_byte(), 0b0001_0000);
        joypad.interrupt_flags.write_byte(0b0000_0000);
        assert_eq!(joypad.internal_buttons.read_byte(), 0b0011_1110);
        assert_eq!(joypad.internal_dpad.read_byte(), 0b0011_1111);
        assert_eq!(joypad.read_byte(), 0b0001_1110);

        // Release A button
        joypad.button_up(A);
        assert_eq!(joypad.interrupt_flags.read_byte(), 0b0000_0000);
        assert_eq!(joypad.internal_buttons.read_byte(), 0b0011_1111);
        assert_eq!(joypad.internal_dpad.read_byte(), 0b0011_1111);
        assert_eq!(joypad.read_byte(), 0b0001_1111);

        // Press Left
        joypad.button_down(Left);
        assert_eq!(joypad.interrupt_flags.read_byte(), 0b0000_0000);
        assert_eq!(joypad.internal_buttons.read_byte(), 0b0011_1111);
        assert_eq!(joypad.internal_dpad.read_byte(), 0b0011_1101);
        assert_eq!(joypad.read_byte(), 0b0001_1111);

        // Press Start
        joypad.button_down(Start);
        assert_eq!(joypad.interrupt_flags.read_byte(), 0b0001_0000);
        joypad.interrupt_flags.write_byte(0b0000_0000);
        assert_eq!(joypad.internal_buttons.read_byte(), 0b0011_0111);
        assert_eq!(joypad.internal_dpad.read_byte(), 0b0011_1101);
        assert_eq!(joypad.read_byte(), 0b0001_0111);

        // Enable reading from dpad
        joypad.write_byte(0b1100_0000);
        assert_eq!(joypad.interrupt_flags.read_byte(), 0b0000_0000);
        assert_eq!(joypad.internal_buttons.read_byte(), 0b0011_0111);
        assert_eq!(joypad.internal_dpad.read_byte(), 0b0011_1101);
        assert_eq!(joypad.read_byte(), 0b0000_0101);

        // Disable reading
        joypad.write_byte(0b0011_0101);
        assert_eq!(joypad.interrupt_flags.read_byte(), 0b0000_0000);
        assert_eq!(joypad.internal_buttons.read_byte(), 0b0011_0111);
        assert_eq!(joypad.internal_dpad.read_byte(), 0b0011_1101);
        assert_eq!(joypad.read_byte(), 0b0011_1111);
    }
}
