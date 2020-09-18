#![allow(dead_code)]
use enum_display_derive::Display;
use process::{RemoteStruct, Signature, SignatureType, UnknownField};
use std::fmt;
use std::fmt::Display;

// 5.2
pub const OFFSET: u64 = 0x1D56860;
pub const SIGNATURE: Signature = Signature {
    bytes: &[
        // ffxiv_dx11.exe+A766A0 - 80 A3 90000000 F8     - and byte ptr [rbx+00000090],-08
        "80", "A3", "90", "00", "00", "00", "F8",
        // ffxiv_dx11.exe+A766A7 - 48 8D 0D B2AF2301     - lea rcx,[ffxiv_dx11.exe+1CB1660]
        "48", "8d", "0d", "B2", "*", "*", "*",
    ],
    sigtype: SignatureType::Relative32 { offset: 0xA },
};

#[repr(u32)]
#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Condition {
    Uninitialized = 0,
    Normal = 1,
    Good = 2,
    Excellent = 3,
    Poor = 4,
    Centered = 5,
    Sturdy = 6,
    Pliant = 7,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum State {
    Uninitialized = 0,
    Unknown1 = 1,
    Unknown2 = 2,
    ReadyForActions = 3,
    CraftSucceeded = 4,
    Unknown5 = 5,
    CraftCanceled = 6,
    Unknown7 = 7,
    CraftFailed = 8,
    CraftActionUsed = 9,
    CraftBuffUsed = 10,
}

impl Default for State {
    fn default() -> State {
        State::Uninitialized
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
            _ => write!(f, "Unknown({})", self.0),
        }
    }
}

impl Default for Action {
    fn default() -> Action {
        Action(0)
    }
}

impl Default for Condition {
    fn default() -> Condition {
        Condition::Uninitialized
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
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

impl fmt::Display for CraftingStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{{\n\t       step: {}, HQ: {}%, condition: {}, state: {:?},",
            self.step, self.hq, self.condition, self.state
        )?;
        writeln!(f, "\t     action: {}", self.action)?;
        writeln!(
            f,
            "\t   progress: {} (last hit: {})",
            self.progress_total, self.progress
        )?;
        writeln!(
            f,
            "\t    quality: {} (last hit: {})",
            self.quality_total, self.quality
        )?;
        writeln!(
            f,
            "\t durability: {} (last hit: {}) }}",
            self.durability, self.last_durability_hit
        )
    }
}

impl fmt::Debug for CraftingStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("unknown 1", &self.__unknown_1)
            .field("unknown 2", &self.__unknown_2)
            .field("unknown 3", &self.__unknown_3)
            .finish()
    }
}

pub type CraftState = RemoteStruct<CraftingStruct>;
