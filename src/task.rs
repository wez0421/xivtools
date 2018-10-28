use crate::garland::Item;
use crate::macros::Action;

// A task represents crafting a specific item a given number of times
// using a provided macro.
#[derive(Debug)]
pub struct Task {
    pub item: Item,           // Item structure for name, id, and materials
    pub count: u64,           // number of items to craft
    pub index: u64,           // index of the recipe if a search returns multiple (default: 0)
    pub actions: Vec<Action>, // List of actions for the task (ie: xiv macro)
    pub gearset: u64,         // Gearset to switch to for crafting
    pub collectable: bool,    // craft collectables
}
