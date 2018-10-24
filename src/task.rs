use crate::macros::Action;

#[derive(Debug)]
pub enum Jobs {
    CUL,
    _ALC,
    _BSM,
    _ARM,
    _GSM,
    _CRP,
    _WVR,
    _LTW,
}

// A task represents crafting a specific item a given number of times
// using a provided macro.
#[derive(Debug)]
pub struct Task {
    pub item_name: String,    // name of the item
    pub job: Jobs,            // job for the item (UNUSED)
    pub collectable: bool,    // craft collectables
    pub count: u64,           // number of items to craft
    pub index: u64,           // index of the recipe if a search returns multiple (default: 0)
    pub actions: Vec<Action>, // List of actions for the task (ie: xiv macro)
    pub gearset: u64,
}
