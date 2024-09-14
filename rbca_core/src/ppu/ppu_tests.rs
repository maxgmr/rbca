use pretty_assertions::assert_eq;

use super::*;

#[test]
fn test_tile_data_start_addr() {
    let mut ppu = PPU::new();

    for tile_num in 0..=127 {
        ppu.lcd_control.set(Lcdc::BGWindowTileDataArea, false);
        assert_eq!(
            ppu.tile_data_start_addr(tile_num),
            0x1000 + ((tile_num as u16) * 16)
        );
        ppu.lcd_control.set(Lcdc::BGWindowTileDataArea, true);
        assert_eq!(ppu.tile_data_start_addr(tile_num), (tile_num as u16) * 16);
    }

    for tile_num in 128..=255 {
        ppu.lcd_control.set(Lcdc::BGWindowTileDataArea, false);
        assert_eq!(
            ppu.tile_data_start_addr(tile_num),
            0x0800 + (((tile_num as u16) - 128) * 16)
        )
    }
}

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
    ppu.lcd_control.set(Lcdc::LcdPpuEnable, true);
    // Set mode to OAM (2)
    ppu.lcd_status.set(Stat::PpuModeBit1, true);
    ppu.lcd_status.set(Stat::PpuModeBit0, false);
    // Advance 70 cycles
    ppu.cycle(70);
    assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 2);
    // Advance 10 cycles (total = 80), so should be in draw mode (3)
    ppu.cycle(10);
    assert_eq!(ppu.read_byte(0xFF41) & 0b0000_0011, 3);
    assert_eq!(ppu.mode_clock, 0);
    // Advance 100 cycles, so should remain in draw mode (3)
    ppu.cycle(100);
    assert_eq!(ppu.get_mode(), 3);
    assert_eq!(ppu.read_byte(0xFF44), 0x00);
    // Advance 76 cycles with only 6-cycle window penalty, so should remain in draw mode (3)
    ppu.cycle(76);
    assert_eq!(ppu.get_mode(), 3);
    assert_eq!(ppu.read_byte(0xFF44), 0x00);
    assert_eq!(ppu.mode_clock, 176);
    // Advance four more cycles to reach HBlank (0)
    ppu.cycle(4);
    assert_eq!(ppu.get_mode(), 0);
    assert_eq!(ppu.read_byte(0xFF44), 0x00);
    assert_eq!(ppu.mode_clock, 2);
    // Advance 196 cycles, so should be back in OAM mode with line advanced (2)
    ppu.cycle(196);
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
    println!("{}", ppu.lcd_y_coord);
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

#[test]
fn test_read_write_vram_oam() {
    let mut ppu = PPU::new();
    assert!(!ppu.lcd_control.get(Lcdc::LcdPpuEnable));
    assert_eq!(ppu.get_mode(), 0);

    fn write_then_read(
        ppu: &mut PPU,
        vram_w_1: u8,
        vram_w_2: u8,
        oam_w_1: u8,
        oam_w_2: u8,
    ) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        ppu.vram[0x0000] = 0x00;
        ppu.vram[0x1FFF] = 0x00;
        ppu.oam[0x0000] = 0x00;
        ppu.oam[0x009F] = 0x00;
        ppu.write_byte(0x8000, vram_w_1);
        ppu.write_byte(0x9FFF, vram_w_2);
        ppu.write_byte(0xFE00, oam_w_1);
        ppu.write_byte(0xFE9F, oam_w_2);
        (
            ppu.vram[0x0000],
            ppu.vram[0x1FFF],
            ppu.oam[0x0000],
            ppu.oam[0x009F],
            ppu.read_byte(0x8000),
            ppu.read_byte(0x9FFF),
            ppu.read_byte(0xFE00),
            ppu.read_byte(0xFE9F),
        )
    }

    // Should be able to read to & write from VRAM & OAM.
    let out = write_then_read(&mut ppu, 0x12, 0x34, 0xAB, 0xCD);
    assert_eq!(out, (0x12, 0x34, 0xAB, 0xCD, 0x12, 0x34, 0xAB, 0xCD));

    // Enable LCD & PPU.
    ppu.lcd_control.set(Lcdc::LcdPpuEnable, true);
    assert!(ppu.lcd_control.get(Lcdc::LcdPpuEnable));
    assert_eq!(ppu.get_mode(), 0);

    // Should be able to read to & write from VRAM & OAM.
    let out = write_then_read(&mut ppu, 0x12, 0x34, 0xAB, 0xCD);
    assert_eq!(out, (0x12, 0x34, 0xAB, 0xCD, 0x12, 0x34, 0xAB, 0xCD));

    // Set mode to OAM.
    ppu.lcd_status.set(Stat::PpuModeBit1, true);
    ppu.lcd_status.set(Stat::PpuModeBit0, false);
    assert_eq!(ppu.get_mode(), 2);

    // Should only be able to access VRAM, not OAM.
    let out = write_then_read(&mut ppu, 0x12, 0x34, 0xAB, 0xCD);
    assert_eq!(out, (0x12, 0x34, 0x00, 0x00, 0x12, 0x34, 0xFF, 0xFF));

    // Disable LCD & PPU.
    ppu.lcd_control.set(Lcdc::LcdPpuEnable, false);
    assert!(!ppu.lcd_control.get(Lcdc::LcdPpuEnable));

    // Should be able to read to & write from VRAM & OAM.
    let out = write_then_read(&mut ppu, 0x12, 0x34, 0xAB, 0xCD);
    assert_eq!(out, (0x12, 0x34, 0xAB, 0xCD, 0x12, 0x34, 0xAB, 0xCD));

    // Set mode to draw pixels.
    ppu.lcd_status.set(Stat::PpuModeBit1, true);
    ppu.lcd_status.set(Stat::PpuModeBit0, true);
    assert_eq!(ppu.get_mode(), 3);

    // Should be able to read to & write from VRAM & OAM.
    let out = write_then_read(&mut ppu, 0x12, 0x34, 0xAB, 0xCD);
    assert_eq!(out, (0x12, 0x34, 0xAB, 0xCD, 0x12, 0x34, 0xAB, 0xCD));

    // Enable LCD & PPU.
    ppu.lcd_control.set(Lcdc::LcdPpuEnable, true);
    assert!(ppu.lcd_control.get(Lcdc::LcdPpuEnable));

    // VRAM & OAM should be inaccessible.
    let out = write_then_read(&mut ppu, 0x12, 0x34, 0xAB, 0xCD);
    assert_eq!(out, (0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF));

    // Set mode to HBlank.
    ppu.lcd_status.set(Stat::PpuModeBit1, false);
    ppu.lcd_status.set(Stat::PpuModeBit0, false);
    assert_eq!(ppu.get_mode(), 0);

    // VRAM & OAM should be accessible.
    let out = write_then_read(&mut ppu, 0x12, 0x34, 0xAB, 0xCD);
    assert_eq!(out, (0x12, 0x34, 0xAB, 0xCD, 0x12, 0x34, 0xAB, 0xCD));

    // Set mode to VBlank.
    ppu.lcd_status.set(Stat::PpuModeBit1, false);
    ppu.lcd_status.set(Stat::PpuModeBit0, true);
    assert_eq!(ppu.get_mode(), 1);

    // VRAM & OAM should be accessible.
    let out = write_then_read(&mut ppu, 0x12, 0x34, 0xAB, 0xCD);
    assert_eq!(out, (0x12, 0x34, 0xAB, 0xCD, 0x12, 0x34, 0xAB, 0xCD));
}
