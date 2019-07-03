use crate::craft::{aaction_add, aaction_remove};
use linked_hash_set::LinkedHashSet;
use log;
use xiv;

const ROLE_ACTIONS: [&str; 18] = [
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
    "piece by piece",
    "rapid synthesis",
    "reclaim",
    "rumination",
    "tricks of the trade",
    "waste not ii",
    "waste not",
];

#[derive(Debug)]
pub struct RoleActions<'a> {
    handle: &'a xiv::XivHandle,
    pub current_actions: LinkedHashSet<String>,
}

// RoleActions is backed by a HashSet using a doubly linked list that can be used
// for LRU-like behavior, ensuring that as we add AdditionalActions they will be older
// actions not referenced in the current macro.
impl<'a> RoleActions<'a> {
    pub fn new(handle: &xiv::XivHandle) -> RoleActions {
        RoleActions {
            handle,
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
            aaction_remove(&self.handle, &old_action);
        }
        log::debug!("adding role action \"{}\"", action);
        aaction_add(&self.handle, action);
    }
}

#[cfg(test)]
mod test {
    use super::RoleActions;
    use xiv;

    #[test]
    fn role_actions() {
        let mut ra = RoleActions::new(&xiv::XivHandle{});
        ra.add_action("tricks of the trade");
        ra.add_action("reclaim");
        assert_eq!(2, ra.count());
        ra.add_action("ingenuity ii");
        ra.add_action("ingenuity");
        ra.add_action("innovation");
        ra.add_action("maker's mark");
        ra.add_action("manipulation");
        ra.add_action("muscle memory");
        ra.add_action("waste not ii");
        ra.add_action("hasty touch");
        assert_eq!(10, ra.count());
        ra.add_action("careful synthesis ii");
        assert_eq!(false, ra.contains("tricks of the trade"));
        assert_eq!(10, ra.count());
        ra.add_action("rumination");
        assert_eq!(false, ra.contains("reclaim"));
        assert_eq!(10, ra.count());
        println!("{:?}", ra);
    }
}
