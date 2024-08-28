//! Functionality related to emulator memory.
use std::default::Default;

/// Memory bus of the Game Boy.
#[derive(Debug)]
pub struct MemoryBus {
    /// The memory contents.
    pub memory: [u8; 0xFFFF],
}
impl MemoryBus {
    /// Create a new [MemoryBus] initialised to zero.
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }
}
