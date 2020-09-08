use std::fmt;

pub enum CityState {
    Limsa,
    Gridania,
    Uldah,
    Foundation,
    Kugane,
    Crystarium,
    Unknown(u8),
}

impl From<u8> for CityState {
    fn from(val: u8) -> Self {
        match val {
            0x1 => CityState::Limsa,
            0x2 => CityState::Gridania,
            0x3 => CityState::Uldah,
            0x4 => CityState::Foundation,
            0x7 => CityState::Kugane,
            0xA => CityState::Crystarium,
            _ => CityState::Unknown(val),
        }
    }
}

impl fmt::Display for CityState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s;
        f.pad(match self {
            CityState::Limsa => "Limsa Lominsa",
            CityState::Gridania => "Gridania",
            CityState::Uldah => "Uldah",
            CityState::Foundation => "Foundation",
            CityState::Kugane => "Kugane",
            CityState::Crystarium => "Crystarium",
            CityState::Unknown(v) => {
                s = format!("Unknown[{}]", v);
                &s
            }
        })
    }
}
