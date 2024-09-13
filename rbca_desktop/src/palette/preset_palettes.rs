use hex_color::HexColor;

use super::Palette;

pub const CLASSIC_GREEN: Palette = Palette {
    lightest: HexColor {
        r: 0x88,
        g: 0xC0,
        b: 0x77,
        a: 0xFF,
    },
    light: HexColor {
        r: 0x4E,
        g: 0xA3,
        b: 0x50,
        a: 0xFF,
    },
    dark: HexColor {
        r: 0x37,
        g: 0x76,
        b: 0x4B,
        a: 0xFF,
    },
    darkest: HexColor {
        r: 0x23,
        g: 0x49,
        b: 0x3A,
        a: 0xFF,
    },
};

pub const WHY_HE_OURPLE: Palette = Palette {
    lightest: HexColor {
        r: 0xCA,
        g: 0xB8,
        b: 0xE3,
        a: 0xFF,
    },
    light: HexColor {
        r: 0x76,
        g: 0x5B,
        b: 0x87,
        a: 0xFF,
    },
    dark: HexColor {
        r: 0x3C,
        g: 0x25,
        b: 0x4A,
        a: 0xFF,
    },
    darkest: HexColor {
        r: 0x1A,
        g: 0x02,
        b: 0x21,
        a: 0xFF,
    },
};

// TODO add more preset palettes
