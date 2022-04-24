use imgui::{im_str, ImStr};
use lazy_static;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Action {
    pub name: &'static str,
    pub wait_ms: u64,
}

// All the current crafting skills in the game minus Collectable Synthesis.
lazy_static::lazy_static! {
    pub static ref ACTIONS: HashMap<&'static str, Action> = {
        let mut h = HashMap::new();
        // Buff actions
        h.insert("advanced touch", Action { name: "Advanced Touch",  wait_ms: 2500 });
        h.insert("basic synthesis", Action { name: "Basic Synthesis",  wait_ms: 2500 });
        h.insert("basic touch", Action { name: "Basic Touch",  wait_ms: 2500 });
        h.insert("byregot's blessing", Action { name: "Byregot's Blessing",  wait_ms: 2500 });
        h.insert("careful observation", Action { name: "Careful Observation",  wait_ms: 2500 });
        h.insert("careful synthesis", Action { name: "Careful Synthesis",  wait_ms: 2500 });
        h.insert("delicate synthesis", Action { name: "Delicate Synthesis",  wait_ms: 2500 });
        h.insert("final appraisal", Action { name: "Final Appraisal",  wait_ms: 1500 });
        h.insert("focused synthesis", Action { name: "Focused Synthesis",  wait_ms: 2500 });
        h.insert("focused touch", Action { name: "Focused Touch",  wait_ms: 2500 });
        h.insert("great strides", Action { name: "Great Strides",  wait_ms: 1500 });
        h.insert("groundwork", Action { name: "Groundwork",  wait_ms: 2500 });
        h.insert("hasty touch", Action { name: "Hasty Touch",  wait_ms: 2500 });
        h.insert("heart and soul", Action { name: "Heart and Soul",  wait_ms: 2500 });
        h.insert("innovation", Action { name: "Innovation",  wait_ms: 1500 });
        h.insert("intensive synthesis", Action { name: "Intensive Synthesis",  wait_ms: 2500 });
        h.insert("manipulation", Action { name: "Manipulation",  wait_ms: 1500 });
        h.insert("master's mend", Action { name: "Master's Mend",  wait_ms: 2500 });
        h.insert("muscle memory", Action { name: "Muscle Memory",  wait_ms: 2500 });
        h.insert("observe", Action { name: "Observe",  wait_ms: 2500 });
        h.insert("precise touch", Action { name: "Precise Touch",  wait_ms: 2500 });
        h.insert("preparatory touch", Action { name: "Preparatory Touch",  wait_ms: 2500 });
        h.insert("prudent synthesis", Action { name: "Prudent Synthesis",  wait_ms: 2500 });
        h.insert("prudent touch", Action { name: "Prudent Touch",  wait_ms: 2500 });
        h.insert("rapid synthesis", Action { name: "Rapid Synthesis",  wait_ms: 2500 });
        h.insert("reflect", Action { name: "Reflect",  wait_ms: 2500 });
        h.insert("standard touch", Action { name: "Standard Touch",  wait_ms: 2500 });
        h.insert("trained eye", Action { name: "Trained Eye",  wait_ms: 2500 });
        h.insert("trained finesse", Action { name: "Trained Finesse",  wait_ms: 2500 });
        h.insert("tricks of the trade", Action { name: "Tricks of the Trade",  wait_ms: 2500 });
        h.insert("veneration", Action { name: "Veneration",  wait_ms: 1500 });
        h.insert("waste not", Action { name: "Waste Not",  wait_ms: 1500 });
        h.insert("waste not ii", Action { name: "Waste Not II",  wait_ms: 1500 });
        h
    };
}
