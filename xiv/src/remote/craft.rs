#![allow(dead_code)]
use process::{RemoteStruct, Signature, SignatureType, UnknownField};
use std::fmt;
use std::fmt::Display;

// 5.2
pub const OFFSET: u64 = 0x1DD6F50;
pub const SIGNATURE: Signature = Signature {
    bytes: &[
        // ffxiv_dx11.exe+A766A0 - 80 A3 90000000 F8     - and byte ptr [rbx+00000090],-08
        "80", "A3", "90", "00", "00", "00", "F8",
        // ffxiv_dx11.exe+A766A7 - 48 8D 0D B2AF2301     - lea rcx,[ffxiv_dx11.exe+1CB1660]
        "48", "8d", "0d", "B2", "*", "*", "*",
    ],
    sigtype: SignatureType::Relative32 { offset: 0xA },
};

// Condition. State, and Action cannot be enums because FFI enums that lack a mapped value will cause a hard crash.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Condition(u32);
impl Condition {
    pub const UNINITIALIZED: Condition = Condition(0x0);
    pub const NORMAL: Condition = Condition(0x1);
    pub const GOOD: Condition = Condition(0x2);
    pub const EXCELLENT: Condition = Condition(0x3);
    pub const POOR: Condition = Condition(0x4);
    pub const CENTERED: Condition = Condition(0x5);
    pub const STURDY: Condition = Condition(0x6);
    pub const PLIANT: Condition = Condition(0x7);
}

impl Default for Condition {
    fn default() -> Self {
        Condition::UNINITIALIZED
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Condition::UNINITIALIZED => write!(f, "Uninitialized"),
            Condition::NORMAL => write!(f, "Normal"),
            Condition::GOOD => write!(f, "Good"),
            Condition::EXCELLENT => write!(f, "Excellent"),
            Condition::POOR => write!(f, "Poor"),
            Condition::CENTERED => write!(f, "Centered"),
            Condition::STURDY => write!(f, "Sturdy"),
            Condition::PLIANT => write!(f, "Pliant"),
            _ => write!(f, "UnknownCondition({:#x})", self.0),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct State(u32);
impl State {
    pub const UNINITIALIZED: State = State(0);
    pub const READY: State = State(0x3);
    pub const SUCCESS: State = State(0x4);
    pub const CANCELED: State = State(0x6);
    pub const FAILED: State = State(0x8);
    pub const ACTION_USED: State = State(0x9);
    pub const BUFF_USED: State = State(0xA);
}

impl Default for State {
    fn default() -> Self {
        State::UNINITIALIZED
    }
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            State::UNINITIALIZED => write!(f, "Uninitialized"),
            State::READY => write!(f, "Ready"),
            State::SUCCESS => write!(f, "Success"),
            State::CANCELED => write!(f, "Canceled"),
            State::FAILED => write!(f, "Failed"),
            State::ACTION_USED => write!(f, "Action used"),
            State::BUFF_USED => write!(f, "Buff used"),
            _ => write!(f, "UnknownState({:#x})", self.0),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Action(u32);
impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            100001 | 100015 | 100030 | 100045 | 100060 | 100075 | 100090 | 100105 => {
                write!(f, "Basic Synthesis")
            }
            100002 | 100016 | 100031 | 100046 | 100061 | 100076 | 100091 | 100106 => {
                write!(f, "Basic Touch")
            }
            100331 | 100332 | 100333 | 100334 | 100335 | 100336 | 100337 | 100338 => {
                write!(f, "Brand of the Elements")
            }
            100339 | 100340 | 100341 | 100342 | 100343 | 100344 | 100345 | 100346 => {
                write!(f, "Byregot's Blessing")
            }
            100395 | 100396 | 100397 | 100398 | 100399 | 100400 | 100401 | 100402 => {
                write!(f, "Careful Observation")
            }
            100203 | 100204 | 100205 | 100206 | 100207 | 100208 | 100209 | 100210 => {
                write!(f, "Careful Synthesis")
            }
            4560 | 4561 | 4562 | 4563 | 4564 | 4565 | 4566 | 4567 => {
                write!(f, "Collectable Synthesis")
            }
            100323 | 100324 | 100325 | 100326 | 100327 | 100328 | 100329 | 100330 => {
                write!(f, "Delicate Synthesis")
            }
            19012 | 19013 | 19014 | 19015 | 19016 | 19017 | 19018 | 19019 => {
                write!(f, "Final Appraisal")
            }
            100235 | 100236 | 100237 | 100238 | 100239 | 100240 | 100241 | 100242 => {
                write!(f, "Focused Synthesis")
            }
            100243 | 100244 | 100245 | 100246 | 100247 | 100248 | 100249 | 100250 => {
                write!(f, "Focused Touch")
            }
            260 | 261 | 262 | 263 | 264 | 265 | 266 | 267 => write!(f, "Great Strides"),
            100403 | 100404 | 100405 | 100406 | 100407 | 100408 | 100409 | 100410 => {
                write!(f, "Groundwork")
            }
            100355 | 100356 | 100357 | 100358 | 100359 | 100360 | 100361 | 100362 => {
                write!(f, "Hasty Touch")
            }
            252 | 253 | 254 | 255 | 256 | 257 | 258 | 259 => write!(f, "Inner Quiet"),
            19004 | 19005 | 19006 | 19007 | 19008 | 19009 | 19010 | 19011 => {
                write!(f, "Innovation")
            }
            100315 | 100316 | 100317 | 100318 | 100319 | 100320 | 100321 | 100322 => {
                write!(f, "Intensive Synthesis")
            }
            4574 | 4575 | 4576 | 4577 | 4578 | 4579 | 4580 | 4581 => write!(f, "Manipulation"),
            100003 | 100017 | 100032 | 100047 | 100062 | 100077 | 100092 | 100107 => {
                write!(f, "Master's Mend")
            }
            100379 | 100380 | 100381 | 100382 | 100383 | 100384 | 100385 | 100386 => {
                write!(f, "Muscle Memory")
            }
            4615 | 4616 | 4617 | 4618 | 4619 | 4620 | 4621 | 4622 => {
                write!(f, "Name of the Elements")
            }
            100010 | 100023 | 100040 | 100053 | 100070 | 100082 | 100099 | 100113 => {
                write!(f, "Observe")
            }
            100219 | 100220 | 100221 | 100222 | 100223 | 100224 | 100225 | 100226 => {
                write!(f, "Patient Touch")
            }
            100128 | 100129 | 100130 | 100131 | 100132 | 100133 | 100134 | 100135 => {
                write!(f, "Precise Touch")
            }
            100299 | 100300 | 100301 | 100302 | 100303 | 100304 | 100305 | 100306 => {
                write!(f, "Preparatory Touch")
            }
            100227 | 100228 | 100229 | 100230 | 100231 | 100232 | 100233 | 100234 => {
                write!(f, "Prudent Touch")
            }
            100363 | 100364 | 100365 | 100366 | 100367 | 100368 | 100369 | 100370 => {
                write!(f, "Rapid Synthesis")
            }
            100387 | 100388 | 100389 | 100390 | 100391 | 100392 | 100393 | 100394 => {
                write!(f, "Reflect")
            }
            100004 | 100018 | 100034 | 100048 | 100064 | 100078 | 100093 | 100109 => {
                write!(f, "Standard Touch")
            }
            100283 | 100284 | 100285 | 100286 | 100287 | 100288 | 100289 | 100290 => {
                write!(f, "Trained Eye")
            }
            100371 | 100372 | 100373 | 100374 | 100375 | 100376 | 100377 | 100378 => {
                write!(f, "Tricks of the Trade")
            }
            19297 | 19298 | 19299 | 19300 | 19301 | 19302 | 19303 | 19304 => {
                write!(f, "Veneration")
            }
            4631 | 4632 | 4633 | 4634 | 4635 | 4636 | 4637 | 4638 => write!(f, "Waste Not"),
            4639 | 4640 | 4641 | 4642 | 4643 | 4644 | 19002 | 19003 => write!(f, "Waste Not II"),
            _ => write!(f, "UnknownAction({:#x})", self.0),
        }
    }
}

impl Default for Action {
    fn default() -> Action {
        Action(0)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CraftingStruct {
    pub state: State, // u32
    __unknown_1: UnknownField<12>,
    pub action: Action, // u32
    __unknown_2: UnknownField<4>,
    pub step: u32,
    pub progress_total: u32,
    pub progress: u32,
    pub quality_total: u32,
    pub quality: u32,
    pub hq: u32,
    pub durability: u32,
    pub last_durability_hit: i32,
    pub condition: Condition, // u32
    __unknown_3: UnknownField<4>,
}

pub type CraftState = RemoteStruct<CraftingStruct>;
