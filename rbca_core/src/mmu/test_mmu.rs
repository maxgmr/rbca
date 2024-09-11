use pretty_assertions::assert_eq;

use super::*;

#[test]
fn test_mmu_read_write() {
    let mut mmu = Mmu::new();
    for address in 0x0000..=0xFFFF {
        let value = (address & 0x00FF) as u8;
        if (0xE000..=0xFDFF).contains(&address) {
            // Attempt to write garbage to ERAM to ensure that it is a duplicate of WRAM
            mmu.write_byte(address, 0xFF);
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
                assert_eq!(mmu.read_byte(address), 0xFF);
                expected_val = 0xFF;
            }
            // Joypad only allows write to bits 4 & 5
            0xFF00 => {
                assert_eq!(mmu.joypad.read_byte() & 0b0011_0000, value & 0b0011_0000);
                expected_val = 0b0000_1111;
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
            0xFF40..=0xFF43 => {
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
