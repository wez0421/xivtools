#![allow(dead_code)]
use process::{RemoteStruct, UnknownField};
use std::borrow::Cow;
use std::ffi::CStr;
use std::fmt;
use std::os::raw::c_char;
const RETAINER_COUNT: usize = 10;

// 5.3
pub const OFFSET: u64 = 0x1d60eb0;

#[derive(Copy, Clone, Debug, Default)]
pub struct RetainerTable {
    pub retainer: [RetainerEntry; RETAINER_COUNT],
    pub display_order: [u8; RETAINER_COUNT],
    pub ready: u8,
    pub total_retainers: u8,
}

enum CityState {
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
        write!(
            f,
            "{}",
            match self {
                CityState::Limsa => "Limsa Lominsa",
                CityState::Gridania => "Gridania",
                CityState::Uldah => "Uldah",
                CityState::Foundation => "Foundation",
                CityState::Kugane => "Kugane",
                CityState::Crystarium => "Crystarium",
                CityState::Unknown(val) => return write!(f, "Unknown({:x})", val),
            }
        )
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RetainerEntry {
    __unknown_1: UnknownField<8>,
    name: [u8; 32],
    pub available: bool,
    pub classjob: u8,
    pub level: u8,
    pub item_count: u8,
    pub gil: u32,
    pub home_city: u8,
    pub market_count: u8,
    __unknown_2: UnknownField<2>,
    pub market_complete: u32,
    pub venture_id: u32,
    pub venture_complete: u32,
    __unknown_3: UnknownField<8>,
}

impl RetainerEntry {
    pub fn name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.name.as_ptr() as *const c_char).to_string_lossy() }
    }
}

pub type Retainers<'a> = RemoteStruct<'a, RetainerTable>;
