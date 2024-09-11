use crate::Flags;

/// Device audio.
// TODO
#[derive(Debug)]
pub struct Audio {
    // Used to alert MMU that the timer triggered some interrupt flags.
    pub interrupt_flags: Flags,
    // 0xFF10
    c1_sweep: Flags,
    // 0xFF11
    c1_len_duty: Flags,
    // 0xFF12
    c1_vol_env: Flags,
    // 0xFF13
    // write-only
    c1_period_low: Flags,
    // 0xFF14
    c1_period_high_control: Flags,
    // 0xFF16
    c2_len_duty: Flags,
    // 0xFF17
    c2_vol_env: Flags,
    // 0xFF18
    // write-only
    c2_period_low: Flags,
    // 0xFF19
    c2_period_high_control: Flags,
    // 0xFF1A
    c3_dac_enable: u8,
    // 0xFF1B
    // write-only
    c3_len_timer: u8,
    // 0xFF1C
    c3_out_level: Flags,
    // 0xFF1D
    // write-only
    c3_period_low: Flags,
    // 0xFF1E
    c3_period_high_control: Flags,
    // 0xFF20
    c4_len_timer: Flags,
    // 0xFF21
    c4_vol_env: Flags,
    // 0xFF22
    c4_freq_rand: Flags,
    // 0xFF23
    c4_control: Flags,
    // 0xFF24
    master_vol_vin_panning: Flags,
    // 0xFF25
    sound_panning: Flags,
    // 0xFF26
    audio_master_control: Flags,
    // 0xFF30-0xFF3F
    wave_pattern_ram: [u8; 0x10],
}
impl Audio {
    /// Create new [Audio].
    pub fn new() -> Self {
        // TODO initial vals
        Self {
            interrupt_flags: Flags::new(0b0000_0000),
            c1_sweep: Flags::new(0b0000_0000),
            c1_len_duty: Flags::new(0b0000_0000),
            c1_vol_env: Flags::new(0b0000_0000),
            c1_period_low: Flags::new(0b0000_0000),
            c1_period_high_control: Flags::new(0b0000_0000),
            c2_len_duty: Flags::new(0b0000_0000),
            c2_vol_env: Flags::new(0b0000_0000),
            c2_period_low: Flags::new(0b0000_0000),
            c2_period_high_control: Flags::new(0b0000_0000),
            c3_dac_enable: 0x00,
            c3_len_timer: 0x00,
            c3_out_level: Flags::new(0b0000_0000),
            c3_period_low: Flags::new(0b0000_0000),
            c3_period_high_control: Flags::new(0b0000_0000),
            c4_len_timer: Flags::new(0b0000_0000),
            c4_vol_env: Flags::new(0b0000_0000),
            c4_freq_rand: Flags::new(0b0000_0000),
            c4_control: Flags::new(0b0000_0000),
            master_vol_vin_panning: Flags::new(0b0000_0000),
            sound_panning: Flags::new(0b0000_0000),
            audio_master_control: Flags::new(0b0000_0000),
            wave_pattern_ram: [0x00; 0x10],
        }
    }

    /// Directly read the byte at the given address.
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF10 => self.c1_sweep.read_byte(),
            0xFF11 => self.c1_len_duty.read_byte(),
            0xFF12 => self.c1_vol_env.read_byte(),
            0xFF13 => 0xFF,
            0xFF14 => self.c1_period_high_control.read_byte(),
            0xFF15 => 0xFF,
            0xFF16 => self.c2_len_duty.read_byte(),
            0xFF17 => self.c2_vol_env.read_byte(),
            0xFF18 => 0xFF,
            0xFF19 => self.c2_period_high_control.read_byte(),
            0xFF1A => self.c3_dac_enable,
            0xFF1B => 0xFF,
            0xFF1C => self.c3_out_level.read_byte(),
            0xFF1D => 0xFF,
            0xFF1E => self.c3_period_high_control.read_byte(),
            0xFF1F => 0xFF,
            0xFF20 => self.c4_len_timer.read_byte(),
            0xFF21 => self.c4_vol_env.read_byte(),
            0xFF22 => self.c4_freq_rand.read_byte(),
            0xFF23 => self.c4_control.read_byte(),
            0xFF24 => self.master_vol_vin_panning.read_byte(),
            0xFF25 => self.sound_panning.read_byte(),
            0xFF26 => self.audio_master_control.read_byte(),
            0xFF30..=0xFF3F => self.wave_pattern_ram[(address - 0xFF30) as usize],
            _ => panic!("Audio: read illegal address {:#06X}.", address),
        }
    }

    /// Directly write to the byte at the given address.
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF10 => self.c1_sweep.write_byte(value),
            0xFF11 => self.c1_len_duty.write_byte(value),
            0xFF12 => self.c1_vol_env.write_byte(value),
            0xFF13 => self.c1_period_low.write_byte(value),
            0xFF14 => self.c1_period_high_control.write_byte(value),
            0xFF15 => {}
            0xFF16 => self.c2_len_duty.write_byte(value),
            0xFF17 => self.c2_vol_env.write_byte(value),
            0xFF18 => self.c2_period_low.write_byte(value),
            0xFF19 => self.c2_period_high_control.write_byte(value),
            0xFF1A => self.c3_dac_enable = value,
            0xFF1B => self.c3_len_timer = value,
            0xFF1C => self.c3_out_level.write_byte(value),
            0xFF1D => self.c3_period_low.write_byte(value),
            0xFF1E => self.c3_period_high_control.write_byte(value),
            0xFF1F => {}
            0xFF20 => self.c4_len_timer.write_byte(value),
            0xFF21 => self.c4_vol_env.write_byte(value),
            0xFF22 => self.c4_freq_rand.write_byte(value),
            0xFF23 => self.c4_control.write_byte(value),
            0xFF24 => self.master_vol_vin_panning.write_byte(value),
            0xFF25 => self.sound_panning.write_byte(value),
            0xFF26 => self.audio_master_control.write_byte(value),
            0xFF30..=0xFF3F => self.wave_pattern_ram[(address - 0xFF30) as usize] = value,
            _ => panic!("Audio: write illegal address {:#06X}.", address),
        }
    }

    /// Perform one audio cycle.
    pub fn cycle(&mut self, _t_cycles: u32) {
        // TODO
    }
}
