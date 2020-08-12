#![allow(dead_code)]
use process::{RemoteStruct, UnknownField};
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;
const RETAINER_COUNT: usize = 10;

// 5.2
pub const OFFSET: u64 = 0x1d61eb0;

#[derive(Copy, Clone, Debug, Default)]
pub struct RetainerTable {
    pub retainer: [RetainerEntry; RETAINER_COUNT],
    pub display_order: [u8; RETAINER_COUNT],
    pub ready: u8,
    pub total_retainers: u8,
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
