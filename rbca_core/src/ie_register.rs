use crate::FlagsEnum;

/// Enum for accessing IE register components.
#[derive(Debug, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum IEHandler {
    /// Joypad interrupt handler
    Joypad,
    /// Serial interrupt handler
    Serial,
    /// Timer interrupt handler
    Timer,
    /// LCD interrupt handler
    LCD,
    /// VBlank interrupt handler
    VBlank,
}
impl FlagsEnum for IEHandler {
    fn val(&self) -> u8 {
        match self {
            Self::Joypad => 0b0001_0000,
            Self::Serial => 0b0000_1000,
            Self::Timer => 0b0000_0100,
            Self::LCD => 0b0000_0010,
            Self::VBlank => 0b0000_0001,
        }
    }
}
