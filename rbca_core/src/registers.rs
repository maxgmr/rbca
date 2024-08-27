//! All functionality related to the registers of the emulated CPU.
use std::default::Default;

/// The registers of the emulated CPU.
#[derive(Debug, Default, Clone)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}
