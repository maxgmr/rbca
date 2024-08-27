//! All functionality related to the registers of the emulated CPU.
use std::default::Default;

/// The registers of the emulated CPU.
#[derive(Debug, Default, Clone)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}
