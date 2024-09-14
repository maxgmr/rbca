use pretty_assertions::assert_eq;

use super::*;

#[test]
fn test_mmu_read_write() {
    let mut mmu = Mmu::new();
    let lcd_status_initial: u8 = mmu.read_byte(0xFF41);
    assert_eq!(lcd_status_initial, 0b1000_0100);

    for address in 0x0000..=0xFFFF {
        let value = (address & 0x00FF) as u8;
        if (0xE000..=0xFDFF).contains(&address) {
            // Attempt to write garbage to ERAM to ensure that it is a duplicate of WRAM
            mmu.write_byte(address, 0xFF);
        } else if address == 0xFF46 {
            // Simulate waiting for the OAM DMA transfer to finish so reads & writes outside WRAM
            // become available again
            mmu.write_byte(address, value);
            mmu.oam_dma_remaining_cycles = 0;
        } else {
            // Write to MMU at all non-illegal addresses
            mmu.write_byte(address, value);
        }

        let expected_val;

        // Ensure correct action was taken by directly accessing destination
        match address {
            0x0000..=0x7FFF => {
                assert_eq!(mmu.cart.rom()[address as usize], value);
                expected_val = value;
            }
            0x8000..=0x9FFF => {
                assert_eq!(mmu.ppu.vram[(address - 0x8000) as usize], value);
                expected_val = value;
            }
            // test cart has no RAM
            0xA000..=0xBFFF => {
                assert_eq!(mmu.cart.read_ram(address), 0xFF);
                expected_val = 0xFF;
            }
            0xC000..=0xDDFF => {
                assert_eq!(mmu.wram[(address - 0xC000) as usize], value);
                assert_eq!(mmu.eram[(address - 0xC000) as usize], value);
                expected_val = value;
            }
            0xDE00..=0xDFFF => {
                assert_eq!(mmu.wram[(address - 0xC000) as usize], value);
                assert!(address as usize > mmu.eram.len() - 1);
                expected_val = value;
            }
            0xE000..=0xFDFF => {
                assert_eq!(
                    mmu.eram[(address - 0xE000) as usize],
                    mmu.wram[(address - 0xE000) as usize]
                );
                expected_val = mmu.wram[(address - 0xE000) as usize];
            }
            0xFE00..=0xFE9F => {
                assert_eq!(mmu.ppu.oam[(address - 0xFE00) as usize], value);
                expected_val = value;
            }
            0xFEA0..=0xFEFF => {
                assert_eq!(mmu.read_byte(address), 0x00);
                expected_val = 0x00;
            }
            // Joypad only allows write to bits 4 & 5
            0xFF00 => {
                assert_eq!(mmu.joypad.read_byte() & 0b0011_0000, value & 0b0011_0000);
                expected_val = 0b1100_1111;
            }
            0xFF01 => {
                assert_eq!(mmu.serial_data, value);
                expected_val = value;
            }
            0xFF02 => {
                assert_eq!(mmu.serial_control.read_byte(), value);
                expected_val = value;
            }
            0xFF03 => {
                assert_eq!(mmu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            // Write to divider sets it to 0x00
            0xFF04 => {
                assert_eq!(mmu.timer.read_byte(address), 0x00);
                expected_val = 0x00;
            }
            0xFF05..=0xFF07 => {
                assert_eq!(mmu.timer.read_byte(address), value);
                expected_val = value;
            }
            0xFF08..=0xFF0E => {
                assert_eq!(mmu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF0F => {
                assert_eq!(mmu.if_reg.read_byte(), value);
                expected_val = value;
            }
            0xFF10..=0xFF12 => {
                assert_eq!(mmu.audio.read_byte(address), value);
                expected_val = value;
            }
            0xFF13 => {
                assert_eq!(mmu.audio.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF14 => {
                assert_eq!(mmu.audio.read_byte(address), value);
                expected_val = value;
            }
            0xFF15 => {
                assert_eq!(mmu.audio.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF16 | 0xFF17 => {
                assert_eq!(mmu.audio.read_byte(address), value);
                expected_val = value;
            }
            0xFF18 => {
                assert_eq!(mmu.audio.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF19 | 0xFF1A => {
                assert_eq!(mmu.audio.read_byte(address), value);
                expected_val = value;
            }
            0xFF1B => {
                assert_eq!(mmu.audio.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF1C => {
                assert_eq!(mmu.audio.read_byte(address), value);
                expected_val = value;
            }
            0xFF1D => {
                assert_eq!(mmu.audio.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF1E => {
                assert_eq!(mmu.audio.read_byte(address), value);
                expected_val = value;
            }
            0xFF1F => {
                assert_eq!(mmu.audio.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF20..=0xFF26 => {
                assert_eq!(mmu.audio.read_byte(address), value);
                expected_val = value;
            }
            0xFF27..=0xFF2F => {
                assert_eq!(mmu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF30..=0xFF3F => {
                assert_eq!(mmu.audio.read_byte(address), value);
                expected_val = value;
            }
            0xFF40 => {
                assert_eq!(mmu.ppu.read_byte(address), value);
                expected_val = value;
            }
            0xFF41 => {
                assert_eq!(mmu.ppu.read_byte(address), 0b0100_0000);
                expected_val = 0b0100_0000;
            }
            0xFF42 | 0xFF43 => {
                assert_eq!(mmu.ppu.read_byte(address), value);
                expected_val = value;
            }
            0xFF44 => {
                if crate::ppu::LY_STUBBED {
                    assert_eq!(mmu.ppu.read_byte(address), 0x90);
                    expected_val = 0x90;
                } else {
                    assert_eq!(mmu.ppu.read_byte(address), 0x00);
                    expected_val = 0x00;
                }
            }
            0xFF45 => {
                assert_eq!(mmu.ppu.read_byte(address), value);
                expected_val = value;
            }
            0xFF46 => {
                assert_eq!(mmu.ppu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF47..=0xFF4B => {
                assert_eq!(mmu.ppu.read_byte(address), value);
                expected_val = value;
            }
            0xFF4C..=0xFF4F => {
                assert_eq!(mmu.ppu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF50 => {
                assert_eq!(mmu.disable_boot_rom, value);
                expected_val = value;
            }
            0xFF51..=0xFF55 => {
                assert_eq!(mmu.ppu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF56..=0xFF67 => {
                assert_eq!(mmu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF68..=0xFF6B => {
                assert_eq!(mmu.ppu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF6C..=0xFF6F => {
                assert_eq!(mmu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF70 => {
                assert_eq!(mmu.ppu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF71..=0xFF7F => {
                assert_eq!(mmu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            0xFF80..=0xFFFE => {
                assert_eq!(mmu.hram[(address as usize) - 0xFF80], value);
                expected_val = value;
            }
            0xFFFF => {
                assert_eq!(mmu.ie_reg.read_byte(), value);
                expected_val = value;
            }
        };

        // Ensure read_byte returns correct value
        assert_eq!(mmu.read_byte(address), expected_val);
    }
}

#[test]
fn test_oam_dma_transfer() {
    let mut mmu = Mmu::new();
    let vals: Vec<u8> = (0x00..=0x9F).collect();
    mmu.wram[0x0000..=0x009F].copy_from_slice(&vals);
    mmu.wram[0x00A0] = 0xFF;

    mmu.write_byte(0xCFAB, 0x00);

    // Should be able to read & write
    assert_eq!(mmu.oam_dma_remaining_cycles, 0);
    mmu.write_byte(0xD000, 0xAB);
    assert_eq!(mmu.read_byte(0xD000), 0xAB);

    // Target = 0xC000; 0xC000 / 0x0100 = 0x00C0
    mmu.oam_dma_transfer(0xC0);
    assert_eq!(mmu.oam_dma_remaining_cycles, 640);

    // Shouldn't be able to read or write anywhere except HRAM
    for address in 0x0000..=0xFF7F {
        mmu.write_byte(address, 0xCD);
        assert_eq!(mmu.read_byte(address), 0xFF);
    }
    mmu.write_byte(0xFFFF, 0xCD);
    assert_eq!(mmu.read_byte(0xFFFF), 0xFF);

    // Should take 640 T-cycles to return to normal
    // take 635 cycles to do 127 checks on all 127 HRAM locations
    for offset in 0..=0x7E_u8 {
        // ensure that HRAM works
        mmu.write_byte(0xFF80 + (offset as u16), offset);
        assert_eq!(mmu.read_byte(0xFF80 + (offset as u16)), offset);

        // ensure that other memory locations are still unusable
        mmu.write_byte(0xC000 + (offset as u16), offset);
        assert_eq!(mmu.read_byte(0xC000 + (offset as u16)), 0xFF);

        mmu.cycle(5);
    }

    // ensure that HRAM works
    mmu.write_byte(0xFF80, 0xAB);
    assert_eq!(mmu.read_byte(0xFF80), 0xAB);

    // ensure that other memory locations are still unusable
    mmu.write_byte(0xC000, 0xCD);
    assert_eq!(mmu.read_byte(0xC000), 0xFF);

    // ensure that OAM DMA goes to 0 on overflow
    mmu.cycle(8);
    assert_eq!(mmu.oam_dma_remaining_cycles, 0);

    // ensure that HRAM still works
    mmu.write_byte(0xFF81, 0xCF);
    assert_eq!(mmu.read_byte(0xFF81), 0xCF);

    // ensure that other memory locations were not written to during OAM DMA
    assert_eq!(mmu.read_byte(0xCFAB), 0x00);

    // ensure that other memory locations now work again
    mmu.write_byte(0xCFAB, 0xDE);
    assert_eq!(mmu.read_byte(0xCFAB), 0xDE);

    mmu.oam_dma_transfer(0xC0);
    assert_eq!(mmu.oam_dma_remaining_cycles, 640);
}
