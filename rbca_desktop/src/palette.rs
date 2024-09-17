use std::default::Default;

use hex_color::HexColor;
use preset_palettes::*;
use sdl2::pixels::Color;

mod preset_palettes;

/// The varieties of preset palettes that can be used.
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub enum PresetPalette {
    #[default]
    ClassicGreen,
    WhyHeOurple,
    MalibuBarbara,
    MintyPie,
}
impl PresetPalette {
    /// Retrieve the static [Palette] that corresponds with this palette preset.
    pub fn get(&self) -> &Palette {
        match *self {
            Self::ClassicGreen => &CLASSIC_GREEN,
            Self::WhyHeOurple => &WHY_HE_OURPLE,
            Self::MalibuBarbara => &MALIBU_BARBARA,
            Self::MintyPie => &MINTY_PIE,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Palette {
    /// The lightest [HexColor] of the palette.
    lightest: HexColor,
    /// The light [HexColor] of the palette.
    light: HexColor,
    /// The dark [HexColor] of the palette.
    dark: HexColor,
    /// The darkest [HexColor] of the palette.
    darkest: HexColor,
}
impl Palette {
    /// Get the lightest [HexColor] of the palette.
    pub fn lightest(&self) -> &HexColor {
        &self.lightest
    }

    /// Get the light [HexColor] of the palette.
    pub fn light(&self) -> &HexColor {
        &self.light
    }

    /// Get the dark [HexColor] of the palette.
    pub fn dark(&self) -> &HexColor {
        &self.dark
    }

    /// Get the darkest [HexColor] of the palette.
    pub fn darkest(&self) -> &HexColor {
        &self.darkest
    }

    /// Get the colour corresponding to the Game Boy internal value.
    /// 0 = lightest
    /// 1 = light
    /// 2 = dark
    /// 3 = darkest
    pub fn num_to_hex(&self, num: u8) -> &HexColor {
        match num {
            1 => self.light(),
            2 => self.dark(),
            3 => self.darkest(),
            _ => self.lightest(),
        }
    }
}
impl Default for Palette {
    fn default() -> Self {
        CLASSIC_GREEN
    }
}

/// Return a given [HexColor] as an [sdl2] pixel [Color].
pub fn hex_to_sdl(hex_color: &HexColor) -> Color {
    Color::RGB(hex_color.r, hex_color.g, hex_color.b)
}
