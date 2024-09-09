use super::{CartFeatures, Cartridge};

/// A ROM-only cartridge.
#[derive(Debug)]
pub struct CartRomOnly {
    rom: Vec<u8>,
    cart_features: CartFeatures,
}
impl CartRomOnly {
    pub fn new(data: Vec<u8>, cart_features: CartFeatures) -> Self {
        Self {
            rom: data,
            cart_features,
        }
    }
}
impl Cartridge for CartRomOnly {
    fn rom(&self) -> &[u8] {
        &self.rom
    }

    fn cart_features(&self) -> &CartFeatures {
        &self.cart_features
    }
}
