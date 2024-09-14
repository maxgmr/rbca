// 2^22 Hz / 4.194304 MHz
pub const SYS_CLOCK_FREQ_HZ: u32 = 4194304;

mod cpu;

use cpu::Cpu;

/// The core emulator controlling everything. Represents the full virtual system.
pub struct Emulator {
    /// The system clock.
    sys_clock: u128,
    /// The CPU.
    cpu: Cpu,
}
impl Emulator {
    /// Advance the system clock one cycle.
    pub fn cycle() {}
}
