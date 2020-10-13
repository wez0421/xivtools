#![allow(dead_code)]
use process::{RemoteStruct, UnknownField};
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;
pub const RETAINER_COUNT: usize = 10;

// 5.31
pub const OFFSET: u64 = 0x1d61eb0;

#[derive(Copy, Clone, Debug, Default)]
pub struct RetainerTable {
    pub retainers: [Retainer; RETAINER_COUNT],
    pub display_order: [u8; RETAINER_COUNT],
    pub ready: u8,
    pub count: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Retainer {
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

impl Retainer {
    pub fn name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.name.as_ptr() as *const c_char).to_string_lossy() }
    }

    pub fn is_valid(&self) -> bool {
        self.level >= 1 && self.available
    }

    pub fn employed(&self) -> bool {
        self.is_valid() && self.venture_id != 0 && self.venture_complete != 0
    }

    pub fn venture(&self) -> crate::Venture {
        crate::Venture::from(self.venture_id)
    }
}

pub type RetainerState = RemoteStruct<RetainerTable>;
