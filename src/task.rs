use crate::macros::Action;

pub enum Jobs {
    CUL,
    ALC,
    BSM,
    ARM,
    GSM,
    CRP,
    WVR,
    LTW,
}

// A task represents crafting a specific item a given number of times
// using a provided macro.
pub struct Task {
    pub item_name: String,    // name of the item
    pub job: Jobs,                // job for the item (UNUSED)
    pub collectable: bool,    // craft collectables
    pub count: u64,           // number of items to craft
    pub index: u64,           // index of the recipe if a search returns multiple (default: 0)
    pub actions: Vec<Action>, // List of actions for the task (ie: xiv macro)
}
