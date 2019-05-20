use crate::craft::{aaction_add, aaction_remove};
use crate::ui::WinHandle;
use linked_hash_set::LinkedHashSet;
use log;

const ROLE_ACTIONS: [&'static str; 32] = [
    "brand of earth",
    "brand of fire",
    "brand of ice",
    "brand of lightning",
    "brand of water",
    "brand of wind",
    "byregot's blessing",
    "careful synthesis ii",
    "careful synthesis",
    "comfort zone",
    "flawless synthesis",
    "hasty touch",
    "ingenuity ii",
    "ingenuity",
    "innovation",
    "maker's mark",
    "manipulation",
    "muscle memory",
    "name of earth",
    "name of fire",
    "name of ice",
    "name of lightning",
    "name of water",
    "name of wind",
    "piece by piece",
    "rapid synthesis",
    "reclaim",
    "rumination",
    "steady hand ii",
    "tricks of the trade",
    "waste not ii",
    "waste not",
];

#[derive(Debug)]
pub struct RoleActions {
    // TODO: Figure out how to push this iterator out to the RoleAction struct
    window: WinHandle,
    pub current_actions: LinkedHashSet<String>,
}

// RoleActions is backed by a HashSet using a doubly linked list that can be used
// for LRU-like behavior, ensuring that as we add AdditionalActions they will be older
// actions not referenced in the current macro.
impl RoleActions {
    pub fn new(window: WinHandle) -> RoleActions {
        RoleActions {
            window,
            current_actions: LinkedHashSet::new(),
        }
    }

    pub fn is_role_action(&self, action: &str) -> bool {
        ROLE_ACTIONS.contains(&&*action.to_lowercase())
    }

    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.current_actions.len()
    }

    #[allow(dead_code)]
    pub fn contains(&self, action: &str) -> bool {
        self.current_actions.contains(action)
    }

    // Returns Some() if the craft engine needs to remove the returned action so that it
    // can add the new one.
    pub fn add_action(&mut self, action: &str) {
        if !self.is_role_action(action) {
            panic!("provided action is not a role action: `{}`", action);
        }

        // If insert returns false then the action was already in the set and no action
        // needs to be taken. It has the side effect of moving it to the back.
        if !self.current_actions.insert(action.to_string()) {
            return;
        }

        // If we now have more than 10 actions we need to remove the front element so there
        // is space for the element we're adding next.
        if self.current_actions.len() > 10 {
            let old_action = self.current_actions.pop_front().unwrap();
            log::debug!("removing role action \"{}\"", old_action);
            aaction_remove(self.window, &old_action);
        }
        log::debug!("adding role action \"{}\"", action);
        aaction_add(self.window, action);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ptr::null_mut;
    #[test]
    fn test_role_actions() {
        let window: WinHandle = null_mut();
        let mut ra = RoleActions::new(window);
        ra.add_action("Tricks of the Trade");
        ra.add_action("Byregot's Blessing");
        ra.add_action("Tricks of the Trade");
        assert_eq!(2, ra.count());
        ra.add_action("ingenuity ii");
        ra.add_action("ingenuity");
        ra.add_action("innovation");
        ra.add_action("maker's mark");
        ra.add_action("manipulation");
        ra.add_action("muscle memory");
        ra.add_action("name of earth");
        ra.add_action("name of fire");
        ra.add_action("name of ice");
        ra.add_action("name of lightning");
        assert_eq!(10, ra.count());
        assert_eq!(false, ra.contains("Tricks of the Trade"));
        assert_eq!(false, ra.contains("Byregot's Blessing"));
        println!("{:?}", ra);
    }
}
