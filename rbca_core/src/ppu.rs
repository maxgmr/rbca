use std::default::Default;

use crate::{mmu::If, Flags, FlagsEnum};

/// For debug: lock read value of lcd_y_coord to 0x90.
pub const LY_STUBBED: bool = false;

/// Width of the Game Boy display in pixels.
pub const DISPLAY_WIDTH: usize = 160;
/// Height of the Game Boy display in pixels.
pub const DISPLAY_HEIGHT: usize = 144;

const OAM_CYCLES: u32 = 80;
const DRAW_PX_CYCLES: u32 = 172;
const HBLANK_CYCLES: u32 = 204;
const VBLANK_CYCLES: u32 = 456;

const MAX_SCANLINES: u8 = 153;

const TILE_MAP_SIZE: usize = 0x0400;
const TILE_DATA_SIZE: usize = 0x1000;

const VRAM_ADDR_OFFSET: u16 = 0x8000;

const WINMAP_START_0: u16 = 0x9800;
const WINMAP_START_1: u16 = 0x9C00;

const BGMAP_START_0: u16 = 0x9800;
const BGMAP_START_1: u16 = 0x9C00;

const TILEDATA_START_0: u16 = 0x8800;
const TILEDATA_START_1: u16 = 0x8000;

/// Pixel processing unit.
#[derive(Debug, Clone)]
pub struct PPU {
    /// Data output of screen.
    pub data_output: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    /// Clone of interrupt flags to keep track of any interrupts set by the PPU.
    pub interrupt_flags: Flags,
    /// 8KiB Video RAM (VRAM).
    pub vram: [u8; 0x2000],
    /// Object attribute memory.
    pub oam: [u8; 0x00A0],
    // Clock to keep track of timing while in a given PPU mode.
    mode_clock: u32,
    /// [0xFF40]
    pub lcd_control: Flags,
    /// [0xFF44] read-only
    lcd_y_coord: u8,
    /// [0xFF45]
    ly_compare: u8,
    /// [0xFF41]
    lcd_status: Flags,
    /// [0xFF42]
    bg_view_y: u8,
    /// [0xFF43]
    bg_view_x: u8,
    /// [0xFF4A]
    win_y: u8,
    /// [0xFF4B]
    win_x: u8,
    /// [0xFF47]
    // DMG mode only
    bg_palette: Flags,
    /// [0xFF48]
    // DMG mode only
    obj_palette_0: Flags,
    /// [0xFF49]
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
        if LY_STUBBED {
            println!("WARNING: rbca is currently LY-stubbed. This means that any read to LY (0xFF44) will return 0x90.");
        }
        Self {
            data_output: [0x00; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            interrupt_flags: Flags::new(0b0000_0000),
            vram: [0x00; 0x2000],
            oam: [0x00; 0x00A0],
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

    /// Directly read the byte of a given address.
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF40 => self.lcd_control.read_byte(),
            0xFF41 => self.lcd_status.read_byte(),
            0xFF42 => self.bg_view_y,
            0xFF43 => self.bg_view_x,
            0xFF44 => {
                if LY_STUBBED {
                    0x90
                } else {
                    self.lcd_y_coord
                }
            }
            0xFF45 => self.ly_compare,
            0xFF46 => 0xFF,
            0xFF47 => self.bg_palette.read_byte(),
            0xFF48 => self.obj_palette_0.read_byte(),
            0xFF49 => self.obj_palette_1.read_byte(),
            0xFF4A => self.win_y,
            0xFF4B => self.win_x,
            // CGB only
            0xFF4C..=0xFF4F => 0xFF,
            0xFF51..=0xFF55 | 0xFF68..=0xFF6B | 0xFF70 => 0xFF,
            _ => panic!("PPU: read illegal address {:#06X}.", address),
        }
    }

    /// Directly replace the byte at the given address. Not recommended.
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = value,
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = value,
            0xFF40 => self.lcd_control.write_byte(value),
            0xFF41 => self.lcd_status.write_byte(value),
            0xFF42 => self.bg_view_y = value,
            0xFF43 => self.bg_view_x = value,
            0xFF44 => {}
            0xFF45 => {
                self.ly_compare = value;
                self.check_lyc_ly();
            }
            0xFF46 => {}
            0xFF47 => self.bg_palette.write_byte(value),
            0xFF48 => self.obj_palette_0.write_byte(value),
            0xFF49 => self.obj_palette_1.write_byte(value),
            0xFF4A => self.win_y = value,
            0xFF4B => self.win_x = value,
            // CGB only
            0xFF4C..=0xFF4F => {}
            0xFF51..=0xFF55 | 0xFF68..=0xFF6B | 0xFF70 => {}
            _ => panic!("PPU: write illegal address {:#06X}.", address),
        }
    }

    /// Advance PPU state equal to the number of t-cycles the CPU advanced.
    pub fn cycle(&mut self, t_cycles: u32) {
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
                    self.render_scanline();
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

                    if self.lcd_y_coord as usize == DISPLAY_HEIGHT {
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
    fn render_scanline(&mut self) {
        // Reset line
        for x in 0..DISPLAY_WIDTH {
            self.set_pixel(x, self.lcd_y_coord as usize, 255);
        }

        self.render_bg_line();
        self.render_obj_line();
    }

    /// Draw the background layer on the LCD.
    fn render_bg_line(&mut self) {
        let bg_map_y = self.lcd_y_coord.wrapping_add(self.bg_view_y);

        // Check whether the current scanline is located within the window.
        let row_is_window =
            self.lcd_control.get(Lcdc::WindowEnable) && (self.lcd_y_coord >= self.win_y);

        for x in 0..(DISPLAY_WIDTH as u8) {
            let bg_map_x = x.wrapping_add(self.bg_view_x);

            // Check whether the current column is located within the window.
            let col_is_window =
                self.lcd_control.get(Lcdc::WindowEnable) && (x >= self.win_x.wrapping_sub(7));

            let px_is_window = row_is_window && col_is_window;

            // Calc index of px in tile data.
            let tile_num = if px_is_window {
                let start = (if self.lcd_control.get(Lcdc::WindowTileMapArea) {
                    WINMAP_START_1
                } else {
                    WINMAP_START_0
                }) - VRAM_ADDR_OFFSET;
                let x_addr_offset = x.wrapping_sub(self.win_x.wrapping_sub(7));
                let y_addr_offset = self.lcd_y_coord.wrapping_sub(self.win_y);
                let addr = Self::get_map_addr(start, x_addr_offset, y_addr_offset);
                self.vram[addr]
            } else {
                let start = (if self.lcd_control.get(Lcdc::BGTileMapArea) {
                    BGMAP_START_1
                } else {
                    BGMAP_START_0
                }) - VRAM_ADDR_OFFSET;
                let addr = Self::get_map_addr(start, bg_map_x, bg_map_y);
                self.vram[addr]
            };

            // Each tile occupies 16 bytes.
            // https://gbdev.io/pandocs/Tile_Data.html
            let tile_data_start_addr =
                if self.lcd_control.get(Lcdc::BGWindowTileDataArea) || tile_num > 0x7F {
                    (tile_num as u16) << 4
                } else {
                    0x1000 | ((tile_num as u16) << 4)
                };

            // Each line of the tile occupies 2 bytes.
            let y_tile_data_addr_offset = ((if px_is_window {
                self.lcd_y_coord - self.win_y
            } else {
                bg_map_y
            } as u16)
                % 8)
                * 2;

            let left_byte_addr = tile_data_start_addr + y_tile_data_addr_offset;
            let right_byte_addr = left_byte_addr + 1;

            let left_byte = self.vram[left_byte_addr as usize];
            let right_byte = self.vram[right_byte_addr as usize];

            // Bit 7 represents leftmost pixel, bit 0 the rightmost pixel.
            let bit_index = if px_is_window {
                self.win_x.wrapping_sub(x) % 8
            } else {
                7 - (bg_map_x % 8)
            };
            // Assemble the colour index from left & right bytes.
            let colour_index = Self::get_colour_index(left_byte, right_byte, bit_index);
            let colour = match colour_index {
                3 => self.bg_palette.read_byte() >> 6,
                2 => (self.bg_palette.read_byte() & 0b0011_0000) >> 4,
                1 => (self.bg_palette.read_byte() & 0b0000_1100) >> 2,
                0 => self.bg_palette.read_byte() & 0b0000_0011,
                _ => unreachable!(
                    "Unreachable colour index {colour_index}. Bad `get_colour_index` function."
                ),
            };
            let data_output_index = ((self.lcd_y_coord as usize) * DISPLAY_WIDTH) + (x as usize);
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
    fn get_map_addr(start: u16, x_offset: u8, y_offset: u8) -> usize {
        (start + (((y_offset as u16) / 8) * 32) + ((x_offset as u16) / 8)) as usize
    }

    /// Draw the sprites layer on the LCD.
    fn render_obj_line(&mut self) {
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
        ((if self.lcd_status.get(Stat::PpuModeBit1) {
            1
        } else {
            0
        }) << 1)
            | (if self.lcd_status.get(Stat::PpuModeBit0) {
                1
            } else {
                0
            })
    }

    /// Set the mode of the PPU.
    fn set_mode(&mut self, value: u8) {
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
            0x1C00
        } else {
            0x1800
        }
    }

    /// Get the start address of the tile data.
    pub fn tile_data_start(&self) -> u16 {
        if self.lcd_control.get(Lcdc::BGWindowTileDataArea) {
            0x0000
        } else {
            0x0800
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_get_mode() {
        let mut ppu = PPU::new();
        ppu.lcd_status.set(Stat::PpuModeBit1, true);
        ppu.lcd_status.set(Stat::PpuModeBit0, true);
        assert_eq!(ppu.get_mode(), 3);

        ppu.lcd_status.set(Stat::PpuModeBit1, true);
        ppu.lcd_status.set(Stat::PpuModeBit0, false);
        assert_eq!(ppu.get_mode(), 2);

        ppu.lcd_status.set(Stat::PpuModeBit1, false);
        ppu.lcd_status.set(Stat::PpuModeBit0, true);
        assert_eq!(ppu.get_mode(), 1);

        ppu.lcd_status.set(Stat::PpuModeBit1, false);
        ppu.lcd_status.set(Stat::PpuModeBit0, false);
        assert_eq!(ppu.get_mode(), 0);
    }

    #[test]
    fn test_cycle_timing() {
        let mut ppu = PPU::new();
        // Check Y coord == 0
        assert_eq!(ppu.read_byte(0xFF44), 0x00);
        // Enable LCD & PPU
        ppu.write_byte(0xFF40, 0b1000_0000 | ppu.read_byte(0xFF40));
        // Set mode to OAM (2)
        ppu.write_byte(0xFF41, 0b0000_0010 | ppu.read_byte(0xFF41));
        ppu.write_byte(0xFF41, !0b0000_0001 & ppu.read_byte(0xFF41));
        // Advance 70 cycles
        ppu.cycle(70);
        assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 2);
        // Advance 10 cycles (total = 80), so should be in draw mode (3)
        ppu.cycle(10);
        assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 3);
        // Advance 100 cycles, so should remain in draw mode (3)
        ppu.cycle(100);
        assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 3);
        assert_eq!(ppu.read_byte(0xFF44), 0x00);
        // Advance 73 cycles without penalties, so should be in hblank mode (0)
        ppu.cycle(73);
        assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 0);
        assert_eq!(ppu.read_byte(0xFF44), 0x00);
        assert_eq!(ppu.mode_clock, 1);
        // Advance 203 cycles, so should be back in OAM mode with line advanced (2)
        ppu.cycle(203);
        assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 2);
        assert_eq!(ppu.read_byte(0xFF44), 0x01);
        assert_eq!(ppu.mode_clock, 0);
        // Advance another line
        ppu.cycle(80);
        ppu.cycle(289);
        ppu.cycle(87);
        assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 2);
        assert_eq!(ppu.read_byte(0xFF44), 0x02);
        assert_eq!(ppu.mode_clock, 0);
        // Advance to last line
        for i in 0..141 {
            ppu.cycle(80);
            ppu.cycle(289);
            ppu.cycle(87);
            assert_eq!(ppu.read_byte(0xFF44), 0x03 + i);
            assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 2);
        }
        // Advance past last line, should be in VBlank mode (1)
        ppu.cycle(80);
        ppu.cycle(289);
        ppu.cycle(87);
        assert_eq!(ppu.read_byte(0xFF44), 144);
        assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 1);
        // Advance partway through first VBlank line
        ppu.cycle(455);
        assert_eq!(ppu.read_byte(0xFF44), 144);
        assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 1);
        // Go to next VBlank line (line 145)
        ppu.cycle(1);
        assert_eq!(ppu.read_byte(0xFF44), 145);
        assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 1);
        // Go to end of VBlank lines
        for i in 0..8 {
            ppu.cycle(456);
            assert_eq!(ppu.read_byte(0xFF44), 146 + i);
            assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 1);
        }
        // Go to end of line 153 & start next frame
        ppu.cycle(456);
        assert_eq!(ppu.read_byte(0xFF44), 0);
        assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 2);
    }
}
