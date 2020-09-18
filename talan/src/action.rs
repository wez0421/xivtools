use imgui::{im_str, ImStr};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Action {
    pub name: &'static str,
    pub gui_name: &'static ImStr,
    pub gcd_ms: u64,
}

// All the current crafting skills in the game minus Collectable Synthesis.
lazy_static::lazy_static! {
    pub static ref ACTIONS: HashMap<&'static str, Action> = {
        let mut h = HashMap::new();
        // Buff actions
        h.insert("final appraisal",
            Action {
                name: "Final Appraisal",
                gui_name: im_str!("Final Appraisal"),
                gcd_ms: 1500
            }
        );
        h.insert("great strides",
            Action {
                name: "Great Strides",
                gui_name: im_str!("Great Strides"),
                gcd_ms: 1500
            }
        );
        h.insert("inner quiet",
            Action {
                name: "Inner Quiet",
                gui_name: im_str!("Inner Quiet"),
                gcd_ms: 1500
            }
        );
        h.insert("innovation",
            Action {
                name: "Innovation",
                gui_name: im_str!("Innovation"),
                gcd_ms: 1500
            }
        );
        h.insert("name of the elements",
            Action {
                name: "Name of the Elements",
                gui_name: im_str!("Name of the Elements"),
                gcd_ms: 1500
            }
        );
        h.insert("waste not ii",
            Action {
                name: "Waste Not II",
                gui_name: im_str!("Waste Not II"),
                gcd_ms: 1500
            }
        );
        h.insert("waste not",
            Action {
                name: "Waste Not",
                gui_name: im_str!("Waste Not"),
                gcd_ms: 1500
            }
        );
        h.insert("veneration",
            Action {
                name: "Veneration",
                gui_name: im_str!("Veneration"),
                gcd_ms: 1500
            }
        );
        // Progress Actions
        h.insert("basic synthesis",
            Action {
                name: "Basic Synthesis",
                gui_name: im_str!("Basic Synthesis"),
                gcd_ms: 2500
            }
        );
        h.insert("brand of the elements",
            Action {
                name: "Brand of the Elements",
                gui_name: im_str!("Brand of the Elements"),
                gcd_ms: 2500
            }
        );
        h.insert("careful synthesis",
            Action {
                name: "Careful Synthesis",
                gui_name: im_str!("Careful Synthesis"),
                gcd_ms: 2500
            }
        );
        h.insert("focused synthesis",
            Action {
                name: "Focused Synthesis",
                gui_name: im_str!("Focused Synthesis"),
                gcd_ms: 2500
            }
        );
        h.insert("groundwork",
            Action {
                name: "Groundwork",
                gui_name: im_str!("Groundwork"),
                gcd_ms: 2500
            }
        );
        h.insert("intensive synthesis",
            Action {
                name: "Intensive Synthesis",
                gui_name: im_str!("Intensive Synthesis"),
                gcd_ms: 2500
            }
        );
        h.insert("muscle memory",
            Action {
                name: "Muscle Memory",
                gui_name: im_str!("Muscle Memory"),
                gcd_ms: 2500
            }
        );
        h.insert("rapid synthesis",
            Action {
                name: "Rapid Synthesis",
                gui_name: im_str!("Rapid Synthesis"),
                gcd_ms: 2500
            }
        );
        // Quality Actions
        h.insert("basic touch",
            Action {
                name: "Basic Touch",
                gui_name: im_str!("Basic Touch"),
                gcd_ms: 2500
            }
        );
        h.insert("byregot's blessing",
            Action {
                name: "Byregot's Blessing",
                gui_name: im_str!("Byregot's Blessing"),
                gcd_ms: 2500
            }
        );
        h.insert("focused touch",
            Action {
                name: "Focused Touch",
                gui_name: im_str!("Focused Touch"),
                gcd_ms: 2500
            }
        );
        h.insert("hasty touch",
            Action {
                name: "Hasty Touch",
                gui_name: im_str!("Hasty Touch"),
                gcd_ms: 2500
            }
        );
        h.insert("patient touch",
            Action {
                name: "Patient Touch",
                gui_name: im_str!("Patient Touch"),
                gcd_ms: 2500
            }
        );
        h.insert("precise touch",
            Action {
                name: "Precise Touch",
                gui_name: im_str!("Precise Touch"),
                gcd_ms: 2500
            }
        );
        h.insert("preparatory touch",
            Action {
                name: "Preparatory Touch",
                gui_name: im_str!("Preparatory Touch"),
                gcd_ms: 2500
            }
        );
        h.insert("prudent touch",
            Action {
                name: "Prudent Touch",
                gui_name: im_str!("Prudent Touch"),
                gcd_ms: 2500
            }
        );
        h.insert("reflect",
            Action {
                name: "Reflect",
                gui_name: im_str!("Reflect"),
                gcd_ms: 2500
            }
        );
        h.insert("standard touch",
            Action {
                name: "Standard Touch",
                gui_name: im_str!("Standard Touch"),
                gcd_ms: 2500
            }
        );
        h.insert("trained eye",
            Action {
                name: "Trained Eye",
                gui_name: im_str!("Trained Eye"),
                gcd_ms: 2500
            }
        );
        // Repair Actions
        h.insert("manipulation",
            Action {
                name: "Manipulation",
                gui_name: im_str!("Manipulation"),
                gcd_ms: 1500
            }
        );
        h.insert("master's mend",
            Action {
                name: "Master's Mend",
                gui_name: im_str!("Master's Mend"),
                gcd_ms: 2500
            }
        );
        // Other Actions
        h.insert("delicate synthesis",
            Action {
                name: "Delicate Synthesis",
                gui_name: im_str!("Delicate Synthesis"),
                gcd_ms: 2500
            }
        );
        h.insert("observe",
            Action {
                name: "Observe",
                gui_name: im_str!("Observe"),
                gcd_ms: 2500
            }
        );
        h.insert("tricks of the trade",
            Action {
                name: "Tricks of the Trade",
                gui_name: im_str!("Tricks of the Trade"),
                gcd_ms: 2500
            }
        );
        h
    };
}
