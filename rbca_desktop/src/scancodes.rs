use sdl2::keyboard::Scancode;
use serde::{
    de::{self, Visitor},
    Deserializer,
};

pub struct ScancodeVisitor;

impl<'de> Visitor<'de> for ScancodeVisitor {
    type Value = Scancode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an SDL2 scancode enum name (https://docs.rs/sdl2/latest/sdl2/keyboard/enum.Scancode.html)")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Scancode::from_name(v).ok_or(E::custom(format!(
            "Scancode \"{}\" is unsupported by SDL2.",
            v
        )))
    }
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Scancode, D::Error> {
    deserializer.deserialize_str(ScancodeVisitor)
}
