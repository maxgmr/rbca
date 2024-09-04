use std::default::Default;

use crate::{Flags, FlagsEnum};

/// Width of the Game Boy display in pixels.
pub const DISPLAY_WIDTH: usize = 160;
/// Height of the Game Boy display in pixels.
pub const DISPLAY_HEIGHT: usize = 144;

/// Picture processing unit.
#[derive(Debug, Clone)]
pub struct PPU {
    /// Set this to true to signal to memory bus to set the right IF register bit.
    pub interrupt_activated: bool,
    // Clock to keep track of timing while in PPU mode.
    mode_clock: u32,
    // [0xFF40]
    lcd_control: Flags,
    // [0xFF44] read-only
    lcd_y_coord: u8,
    // [0xFF45]
    ly_compare: u8,
    // [0xFF41]
    lcd_status: Flags,
    // [0xFF42]
    bg_view_y: u8,
    // [0xFF43]
    bg_view_x: u8,
    // [0xFF4A]
    win_y: u8,
    // [0xFF4B]
    win_x: u8,
    // [0xFF47]
    // DMG mode only
    bg_palette_data: Flags,
    // [0xFF48]
    // DMG mode only
    obj_palette_0: Flags,
    // [0xFF49]
    // DMG mode only
    obj_palette_1: Flags,
    // [0xFF68]
    // TODO CGB mode only
    // bg_palette_index: Flags,
    // [0xFF69]
    // TODO CGB mode only
    // bg_palette_data_cgb0: Flags,
    // [0xFF70]
    // TODO CGB mode only
    // bg_palette_data_cgb1: Flags,
    // [0xFF6A]
    // TODO CGB mode only
    // obj_palette_index: Flags,
    // [0xFF6B]
    // TODO CGB mode only
    // obj_palette_data: Flags,
}
impl PPU {
    /// Create a new PPU.
    pub fn new() -> Self {
        Self {
            interrupt_activated: false,
            mode_clock: 0,
            lcd_control: Flags::new(0b0101_1000),
            lcd_y_coord: 0b0000_0000,
            ly_compare: 0b0000_0000,
            lcd_status: Flags::new(0b1000_0100),
            bg_view_y: 0b0000_0000,
            bg_view_x: 0b0000_0000,
            win_y: 0b0000_0000,
            win_x: 0b0000_0000,
            bg_palette_data: Flags::new(0b0000_0000),
            obj_palette_0: Flags::new(0b0000_0000),
            obj_palette_1: Flags::new(0b0000_0001),
            // bg_palette_index: Flags::new(0b0000_0000),
            // bg_palette_data_cgb0: Flags::new(0b0000_0000),
            // bg_palette_data_cgb1: Flags::new(0b0000_0000),
            // obj_palette_index: Flags::new(0b0000_0000),
            // obj_palette_data: Flags::new(0b0000_0000),
        }
    }

    /// Directly retrieve the byte at the given address. Not recommended.
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000 => self.lcd_control.read_byte(),
            0x0001 => self.lcd_status.read_byte(),
            0x0002 => self.bg_view_y,
            0x0003 => self.bg_view_x,
            0x0004 => self.lcd_y_coord,
            0x0005 => self.ly_compare,
            // 0x0006 => oam dma transfer???
            0x0007 => self.bg_palette_data.read_byte(),
            0x0008 => self.obj_palette_0.read_byte(),
            0x0009 => self.obj_palette_1.read_byte(),
            0x000A => self.win_y,
            0x000B => self.win_x,
            _ => unimplemented!("Unimplemented PPU address: {:#04X}", address),
        }
    }

    /// Directly replace the byte at the given address. Not recommended.
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000 => self.lcd_control.write_byte(value),
            0x0001 => self.lcd_status.write_byte(value),
            0x0002 => self.bg_view_y = value,
            0x0003 => self.bg_view_x = value,
            0x0004 => self.lcd_y_coord = value,
            0x0005 => self.ly_compare = value,
            // 0x0006 => oam dma transfer???
            0x0007 => self.bg_palette_data.write_byte(value),
            0x0008 => self.obj_palette_0.write_byte(value),
            0x0009 => self.obj_palette_1.write_byte(value),
            0x000A => self.win_y = value,
            0x000B => self.win_x = value,
            _ => unimplemented!("Unimplemented PPU address: {:#04X}", address),
        }
    }

    /// Perform one full cycle.
    // TODO
    pub fn cycle(&mut self, t_cycles: u32) {
        if !self.lcd_control.get(Lcdc::LcdPpuEnable) {
            return;
        }

        match self.mode() {
            // OAM scan. Search for OBJs which overlap this line. Scanline active. OAM (except by
            // DMA) inaccessible.
            2 => {}
            // Drawing pixels. Send pixels to the LCD. OAM (except by DMA) & VRAM inaccessible.
            3 => {}
            // HBlank. After the last HBlank, push the screen data to the canvas.
            0 => {}
            // VBlank. Wait until next frame.
            1 => {}
            _ => {}
        }
    }

    /// Get the mode of the PPU.
    pub fn mode(&self) -> u8 {
        (if self.lcd_status.get(Stat::PpuModeBit1) {
            0x02
        } else {
            0x00
        } | if self.lcd_status.get(Stat::PpuModeBit0) {
            0x01
        } else {
            0x00
        })
    }
}
impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

/// LCD control enum.
pub enum Lcdc {
    LcdPpuEnable,
    WindowTileMapArea,
    WindowEnable,
    BGWindowTileDataArea,
    BGTileMapArea,
    OBJSize,
    OBJEnable,
    BGWindowEnablePriority,
}
impl FlagsEnum for Lcdc {
    fn val(&self) -> u8 {
        match self {
            Lcdc::LcdPpuEnable => 0b1000_0000,
            Lcdc::WindowTileMapArea => 0b0100_0000,
            Lcdc::WindowEnable => 0b0010_0000,
            Lcdc::BGWindowTileDataArea => 0b0001_0000,
            Lcdc::BGTileMapArea => 0b0000_1000,
            Lcdc::OBJSize => 0b0000_0100,
            Lcdc::OBJEnable => 0b0000_0010,
            Lcdc::BGWindowEnablePriority => 0b0000_0001,
        }
    }
}

/// LCD status enum.
pub enum Stat {
    LycIntSelect,
    Mode2IntSelect,
    Mode1IntSelect,
    Mode0IntSelect,
    // read-only
    LycEqLy,
    // read-only
    PpuModeBit1,
    // read-only
    PpuModeBit0,
}
impl FlagsEnum for Stat {
    fn val(&self) -> u8 {
        match self {
            Stat::LycIntSelect => 0b0100_0000,
            Stat::Mode2IntSelect => 0b0010_0000,
            Stat::Mode1IntSelect => 0b0001_0000,
            Stat::Mode0IntSelect => 0b0000_1000,
            Stat::LycEqLy => 0b0000_0100,
            Stat::PpuModeBit1 => 0b0000_0010,
            Stat::PpuModeBit0 => 0b0000_0001,
        }
    }
}

/// Color ID status enum.
/// 0b00 = white
/// 0b01 = light grey
/// 0b10 = dark grey
/// 0b11 = black
pub enum ColorID {
    ID3Bit1,
    ID3Bit0,
    ID2Bit1,
    ID2Bit0,
    ID1Bit1,
    ID1Bit0,
    // ignored for OBJ
    ID0Bit1,
    // ignored for OBJ
    ID0Bit0,
}
impl FlagsEnum for ColorID {
    fn val(&self) -> u8 {
        match self {
            ColorID::ID3Bit1 => 0b1000_0000,
            ColorID::ID3Bit0 => 0b0100_0000,
            ColorID::ID2Bit1 => 0b0010_0000,
            ColorID::ID2Bit0 => 0b0001_0000,
            ColorID::ID1Bit1 => 0b0000_1000,
            ColorID::ID1Bit0 => 0b0000_0100,
            ColorID::ID0Bit1 => 0b0000_0010,
            ColorID::ID0Bit0 => 0b0000_0001,
        }
    }
}

/// Background colour palette index
pub enum Bcps {
    AutoInc,
    AddrBit5,
    AddrBit4,
    AddrBit3,
    AddrBit2,
    AddrBit1,
    AddrBit0,
}
impl FlagsEnum for Bcps {
    fn val(&self) -> u8 {
        match self {
            Bcps::AutoInc => 0b1000_0000,
            Bcps::AddrBit5 => 0b0010_0000,
            Bcps::AddrBit4 => 0b0001_0000,
            Bcps::AddrBit3 => 0b0000_1000,
            Bcps::AddrBit2 => 0b0000_0100,
            Bcps::AddrBit1 => 0b0000_0010,
            Bcps::AddrBit0 => 0b0000_0001,
        }
    }
}

/// Colour palette data 0 (bits 0..=7)
pub enum Cpd0 {
    G2,
    G1,
    G0,
    R4,
    R3,
    R2,
    R1,
    R0,
}
impl FlagsEnum for Cpd0 {
    fn val(&self) -> u8 {
        match self {
            Cpd0::G2 => 0b1000_0000,
            Cpd0::G1 => 0b0100_0000,
            Cpd0::G0 => 0b0010_0000,
            Cpd0::R4 => 0b0001_0000,
            Cpd0::R3 => 0b0000_1000,
            Cpd0::R2 => 0b0000_0100,
            Cpd0::R1 => 0b0000_0010,
            Cpd0::R0 => 0b0000_0001,
        }
    }
}

/// Colour palette data 1 (bits 8..=15)
pub enum Cpd1 {
    B4,
    B3,
    B2,
    B1,
    B0,
    G4,
    G3,
}
impl FlagsEnum for Cpd1 {
    fn val(&self) -> u8 {
        match self {
            Cpd1::B4 => 0b0100_0000,
            Cpd1::B3 => 0b0010_0000,
            Cpd1::B2 => 0b0001_0000,
            Cpd1::B1 => 0b0000_1000,
            Cpd1::B0 => 0b0000_0100,
            Cpd1::G4 => 0b0000_0010,
            Cpd1::G3 => 0b0000_0001,
        }
    }
}
