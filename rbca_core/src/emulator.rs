/// All components of the virtual hardware.
use super::{Cpu, MemoryBus};

#[derive(Debug, Default)]
pub struct Emulator {
    cpu: Cpu,
    mem_bus: MemoryBus,
}
impl Emulator {}
