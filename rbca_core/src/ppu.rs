use std::default::Default;

use crate::{io_registers::If, Flags, FlagsEnum};

/// Width of the Game Boy display in pixels.
pub const DISPLAY_WIDTH: usize = 160;
/// Height of the Game Boy display in pixels.
pub const DISPLAY_HEIGHT: usize = 144;

const OAM_CYCLES: u32 = 80;
const DRAW_PX_CYCLES: u32 = 172;
const HBLANK_CYCLES: u32 = 204;
const VBLANK_CYCLES: u32 = 456;

const MAX_SCANLINES: u8 = 153;

/// Picture processing unit.
#[derive(Debug, Clone)]
pub struct PPU {
    /// Data output of screen.
    pub data_output: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT * 3],
    /// Clone of interrupt flags to keep track of any interrupts set by the PPU.
    pub interrupt_flags: Flags,
    // Clock to keep track of timing while in a given PPU mode.
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
    bg_palette: Flags,
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
            data_output: [0x00; DISPLAY_WIDTH * DISPLAY_HEIGHT * 3],
            interrupt_flags: Flags::new(0b0000_0000),
            mode_clock: 0,
            lcd_control: Flags::new(0b0101_1000),
            lcd_y_coord: 0b0000_0000,
            ly_compare: 0b0000_0000,
            lcd_status: Flags::new(0b1000_0100),
            bg_view_y: 0b0000_0000,
            bg_view_x: 0b0000_0000,
            win_y: 0b0000_0000,
            win_x: 0b0000_0000,
            bg_palette: Flags::new(0b0000_0000),
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
            0x0007 => self.bg_palette.read_byte(),
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
            0x0005 => {
                self.ly_compare = value;
                self.check_lyc_ly();
            }
            // 0x0006 => oam dma transfer???
            0x0007 => self.bg_palette.write_byte(value),
            0x0008 => self.obj_palette_0.write_byte(value),
            0x0009 => self.obj_palette_1.write_byte(value),
            0x000A => self.win_y = value,
            0x000B => self.win_x = value,
            _ => unimplemented!("Unimplemented PPU address: {:#04X}", address),
        }
    }

    /// Advance PPU state equal to the number of t-cycles the CPU advanced.
    pub fn cycle(
        &mut self,
        t_cycles: u32,
        bg_map: &[u8; 0x0400],
        win_map: &[u8; 0x0400],
        tile_data: &[u8; 0x2000],
    ) {
        if !self.lcd_control.get(Lcdc::LcdPpuEnable) {
            return;
        }

        self.mode_clock += t_cycles;

        match self.get_mode() {
            // OAM scan. Search for OBJs which overlap this line. Scanline active. OAM (except by
            // DMA) inaccessible.
            2 => {
                if self.mode_clock >= OAM_CYCLES {
                    // Enter drawing pixels mode
                    self.mode_clock %= OAM_CYCLES;
                    self.set_mode(3);
                }
            }
            // Drawing pixels. Send pixels to the LCD. OAM (except by DMA) & VRAM inaccessible.
            3 => {
                if self.mode_clock >= DRAW_PX_CYCLES {
                    self.render_scanline(bg_map, win_map, tile_data);
                    // Enter HBlank mode
                    if self.lcd_status.get(Stat::Mode0IntSelect) {
                        self.interrupt_flags.set(If::Lcd, true);
                    }
                    self.mode_clock %= DRAW_PX_CYCLES;
                    self.set_mode(0);
                }
            }
            // HBlank. After the last HBlank, push the screen data to the canvas.
            0 => {
                if self.mode_clock >= HBLANK_CYCLES {
                    self.mode_clock %= HBLANK_CYCLES;
                    self.lcd_y_coord += 1;

                    if self.lcd_y_coord as usize == (DISPLAY_HEIGHT - 1) {
                        // Lines done. Set VBlank interrupt & enter VBlank mode
                        self.interrupt_flags.set(If::VBlank, true);
                        if self.lcd_status.get(Stat::Mode1IntSelect) {
                            self.interrupt_flags.set(If::Lcd, true);
                        }
                        self.set_mode(1);
                    } else {
                        // Still more lines. Go to next line.
                        if self.lcd_status.get(Stat::Mode2IntSelect) {
                            self.interrupt_flags.set(If::Lcd, true);
                        }
                        self.set_mode(2);
                    }
                }
            }
            // VBlank. Wait until next frame.
            1 => {
                if self.mode_clock >= VBLANK_CYCLES {
                    self.mode_clock %= VBLANK_CYCLES;
                    self.lcd_y_coord += 1;

                    self.check_lyc_ly();

                    if self.lcd_y_coord > MAX_SCANLINES {
                        // Restart scanning modes
                        if self.lcd_status.get(Stat::Mode2IntSelect) {
                            self.interrupt_flags.set(If::Lcd, true);
                        }
                        self.set_mode(2);
                        self.lcd_y_coord = 0;
                    }
                }
            }
            _ => {}
        }
    }

    /// Render a line of pixels on the LCD.
    fn render_scanline(
        &mut self,
        bg_map: &[u8; 0x0400],
        win_map: &[u8; 0x0400],
        tile_data: &[u8; 0x2000],
    ) {
        // Reset line
        for x in 0..DISPLAY_WIDTH {
            self.set_pixel(x, self.lcd_y_coord as usize, 255);
        }
        self.render_bg_line(bg_map, win_map, tile_data);
        self.render_obj_line(bg_map, win_map, tile_data);
    }

    /// Draw the background layer on the LCD.
    fn render_bg_line(
        &mut self,
        bg_map: &[u8; 0x0400],
        win_map: &[u8; 0x0400],
        tile_data: &[u8; 0x2000],
    ) {
        let bg_map_y = self.lcd_y_coord.wrapping_add(self.bg_view_y);

        // Check whether the current scanline is located within the window.
        let row_is_window =
            self.lcd_control.get(Lcdc::WindowEnable) && (self.lcd_y_coord >= self.win_y);

        for x in 0..(DISPLAY_WIDTH as u8) {
            let bg_map_x = x.wrapping_add(self.bg_view_x);

            // Check whether the current column is located within the window.
            let col_is_window =
                self.lcd_control.get(Lcdc::WindowEnable) && (x >= self.win_x.wrapping_sub(7));

            let is_window = row_is_window && col_is_window;

            let tile_num = if is_window {
                let x_addr_offset = x.wrapping_sub(self.win_x.wrapping_sub(7));
                let y_addr_offset = self.lcd_y_coord.wrapping_sub(self.win_y);
                let addr = Self::get_map_addr(x_addr_offset, y_addr_offset);
                win_map[addr]
            } else {
                let addr = Self::get_map_addr(bg_map_x, bg_map_y);
                bg_map[addr]
            };

            // Each tile occupies 16 bytes.
            // https://gbdev.io/pandocs/Tile_Data.html
            let tile_data_start_addr =
                if self.lcd_control.get(Lcdc::BGWindowTileDataArea) || tile_num > 0x7F {
                    (tile_num as u16) << 4
                } else {
                    0x1000 | ((tile_num as u16) * 16)
                };

            // Each line of the tile occupies 2 bytes.
            let y_tile_data_addr_offset = ((if is_window {
                self.lcd_y_coord - self.win_y
            } else {
                bg_map_y
            } as u16)
                % 8)
                * 2;

            let left_byte_addr = tile_data_start_addr + y_tile_data_addr_offset;
            let right_byte_addr = left_byte_addr + 1;

            let left_byte = tile_data[left_byte_addr as usize];
            let right_byte = tile_data[right_byte_addr as usize];

            // Bit 7 represents leftmost pixel, bit 0 the rightmost pixel.
            let bit_index = if is_window {
                self.win_x.wrapping_sub(x) % 8
            } else {
                7 - (bg_map_x % 8)
            };
            // Assemble the colour index from left & right bytes.
            let colour_index = Self::get_colour_index(left_byte, right_byte, bit_index);
            let colour = match (colour_index & 0b10, colour_index & 0b01) {
                (1, 1) => self.bg_palette.read_byte() >> 6,
                (1, 0) => (self.bg_palette.read_byte() & 0b0011_0000) >> 4,
                (0, 1) => (self.bg_palette.read_byte() & 0b0000_1100) >> 2,
                _ => self.bg_palette.read_byte() & 0b0000_0011,
            };
            let offset = self.lcd_y_coord as usize + (256 * x as usize);
        }
    }

    fn get_colour_index(left_byte: u8, right_byte: u8, bit_index: u8) -> u8 {
        let bit_0 = if left_byte & (1 << bit_index) > 0 {
            1
        } else {
            0
        };
        let bit_1 = if right_byte & (1 << bit_index) > 0 {
            1
        } else {
            0
        };
        (bit_1 << 1) | bit_0
    }

    /// Get the tile map address based on the X & Y offsets.
    fn get_map_addr(x_offset: u8, y_offset: u8) -> usize {
        ((((y_offset as u16) / 8) * 32) + ((x_offset as u16) / 8)) as usize
    }

    /// Draw the sprites layer on the LCD.
    fn render_obj_line(
        &mut self,
        bg_map: &[u8; 0x0400],
        win_map: &[u8; 0x0400],
        tile_data: &[u8; 0x2000],
    ) {
        if !self.lcd_control.get(Lcdc::OBJEnable) {
            return;
        }

        // TODO
    }

    /// Set the colour of a pixel.
    fn set_pixel(&mut self, x: usize, y: usize, colour: u8) {
        let pixel_index = (y * DISPLAY_WIDTH * 3) + (x * 3);
        self.data_output[pixel_index] = colour;
        self.data_output[pixel_index + 1] = colour;
        self.data_output[pixel_index + 2] = colour;
    }

    /// If LYC int select and LYC == LY, activate the LCD status interrupt.
    fn check_lyc_ly(&mut self) {
        if self.lcd_status.get(Stat::LycIntSelect) && (self.lcd_y_coord == self.ly_compare) {
            self.interrupt_flags.set(If::Lcd, true);
        }
    }

    /// Get the mode of the PPU.
    pub fn get_mode(&self) -> u8 {
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

    /// Set the mode of the PPU.
    pub fn set_mode(&mut self, value: u8) {
        self.lcd_status.set(Stat::PpuModeBit1, (value & 0b10) != 0);
        self.lcd_status.set(Stat::PpuModeBit0, (value & 0b01) != 0);
    }

    /// Get the start address of the background tile map.
    pub fn bg_map_start(&self) -> u16 {
        Self::map_start_helper(self.lcd_control.get(Lcdc::BGTileMapArea))
    }
    /// Get the start address of the window tile map.
    pub fn win_map_start(&self) -> u16 {
        Self::map_start_helper(self.lcd_control.get(Lcdc::WindowTileMapArea))
    }
    fn map_start_helper(value: bool) -> u16 {
        if value {
            0x9C00
        } else {
            0x9800
        }
    }

    /// Get the start address of the tile data.
    pub fn tile_data_start(&self) -> u16 {
        if self.lcd_control.get(Lcdc::BGWindowTileDataArea) {
            0x8000
        } else {
            0x8800
        }
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
