use xivapi;

#[derive(Copy, Clone, Debug)]
pub struct MaterialCount {
    pub nq: i32,
    pub hq: i32,
}

// A task represents crafting a specific item a given number of times
// using a provided recipe and macro. mat_quality is a specific field
// separate from Recipe because the Recipe type is from an external
// crate.
#[derive(Debug)]
pub struct Task {
    pub quantity: i32,        // number of items to craft
    pub is_collectable: bool, // craft collectables
    pub recipe: xivapi::Recipe,
    pub mat_quality: Vec<MaterialCount>,
    pub macro_id: i32,
}
