use crate::macros::Macro;
use crate::recipe::Recipe;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct MaterialCount {
    pub nq: u32,
    pub hq: u32,
}

// A task represents crafting a specific item a given number of times
// using a provided recipe and macro.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Task {
    pub specify_materials: bool,
    pub macro_id: usize,
    pub mat_quality: Vec<MaterialCount>,
    pub quantity: u32, // number of items to craft
    pub recipe: Recipe,
    #[serde(default)]
    pub estimate: u32,
}

impl Task {
    pub fn new(recipe: Recipe, count: u32) -> Task {
        Task {
            specify_materials: false,
            macro_id: 0,
            quantity: count,
            mat_quality: recipe
                .mats
                .iter()
                .map(|m| MaterialCount { nq: m.count, hq: 0 })
                .collect(),
            recipe,
            estimate: 0,
        }
    }

    pub fn update_estimate(&mut self, macros: &[Macro]) {
        // 5 extra seconds of padding per craft is to conservatively cover the UI navigation per item.
        self.estimate = self.quantity
            * (macros[self.macro_id]
                .actions
                .iter()
                .fold(0, |acc, action| acc + action.wait_ms) as u32
                + 5000);
    }
}

// Used to represent the status of a Task being executed by the crafting
// engine.
#[derive(Clone, Debug)]
pub struct Status {
    pub name: String,
    pub finished: u32,
    pub total: u32,
}

impl<'a> From<&'a Task> for Status {
    fn from(task: &'a Task) -> Self {
        Status {
            name: task.recipe.name.clone(),
            finished: 0,
            total: task.quantity as u32,
        }
    }
}
