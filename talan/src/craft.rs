use crate::macros;
use crate::role_actions::RoleActions;
use crate::task::Task;
use log;
use xiv::ui;

// Runs through the set of tasks
// TODO: make it actually run more than one task
pub fn craft_items(handle: &xiv::XivHandle, tasks: &[Task]) {
    // TODO: this will be a problem when we run multiple tasks
    // TODO: Investigate why there's always a longer delay after Careful Synthesis II
    // TODO: Tea is going to be a problem for non-specialty recipes
    let mut role_actions = RoleActions::new(handle);
    // Clear role actions before we iterate tasks so the game state
    // and role action state will be in sync.
    let mut gearset: u64 = 0;
    for task in tasks {
        // Change to the appropriate job if one is set. XIV
        // gearsets start at 1, so 0 is a safe empty value.
        if task.gearset > 0 && task.gearset != gearset {
            // If we're changing jobs we need to set role actions up again,
            // otherwise there's a good chance we can reuse some of the role
            // actions we already have for the next craft
            change_gearset(handle, task.gearset);
            gearset = task.gearset;
        }

        aaction_clear(handle);
        ui::clear_window(handle);
        if task.collectable {
            toggle_collectable(handle);
        }

        // Check the role action cache and configure any we need for this task
        configure_role_actions(&mut role_actions, task);

        // Bring up the crafting window itself and give it time to appear
        ui::open_craft_window(handle);

        // Navigate to the correct recipe based on the index provided
        select_recipe(handle, &task);
        // Time to craft the items
        execute_task(handle, &task);

        // Close out of the crafting window and stand up
        ui::clear_window(handle);
        if task.collectable {
            toggle_collectable(handle);
        }
    }
}

fn configure_role_actions(role_actions: &mut RoleActions, task: &Task) {
    for action in &task.actions {
        if role_actions.is_role_action(&action.name) {
            role_actions.add_action(&action.name);
        }
    }
}

// Selects the appropriate recipe then leaves the cursor on the Synthesize
// button, ready for material selection.
fn select_recipe(handle: &xiv::XivHandle, task: &Task) {
    log::info!("selecting recipe...");
    // Loop backward through the UI 9 times to ensure we hit the text box
    // no matter what crafting class we are. The text input boxes are strangely
    // modal so that if we select them at any point they will hold on to focus
    // for characters.
    //
    // TODO: Link recipe job to this so we don't move more than we need
    for _ in 0..9 {
        ui::cursor_backward(handle);
    }

    ui::press_confirm(handle);
    ui::send_string(handle, &task.item.name);
    ui::press_enter(handle);

    // Navigate to the offset we need
    for _ in 0..task.index {
        xiv::cursor_down(handle);
    }

    // Select the recipe to get to components / sythen
    xiv::press_confirm(handle);
}

fn select_materials(handle: &xiv::XivHandle, task: &Task) {
    log::info!("selecting materials...");
    xiv::cursor_up(handle);
    // TODO implement HQ > NQ
    ui::cursor_right(handle);
    ui::cursor_right(handle);

    // The cursor should be on the quantity field of the bottom item now
    // We move through the ingredients backwards because we start at the bottom of t
    for (i, material) in task.item.materials.iter().enumerate().rev() {
        log::trace!("{}x {}", material.count, material.name);
        for _ in 0..material.count {
            ui::press_confirm(handle)
        }
        // Don't move up if we've made it back to the top of the ingredients
        if i != 0 {
            ui::cursor_up(handle);
        }
    }
    ui::cursor_left(handle);
    for material in &task.item.materials {
        for _ in 0..material.count {
            ui::press_confirm(handle)
        }
        ui::cursor_down(handle);
    }
}

fn execute_task(handle: &xiv::XivHandle, task: &Task) {
    for task_index in 1..=task.count {
        println!("crafting {} {}/{}", task.item.name, task_index, task.count);
        // If we're at the start of a task we will already have the Synthesize button
        // selected with the pointer.
        select_materials(handle, &task);
        ui::press_confirm(handle);
        // Wait for the craft dialog to pop up
        // XXX need a wait here
        // and now execute the actions
        execute_actions(handle, &task.actions);

        // There are two paths here. If an item is collectable then it will
        // prompt a dialog to collect the item as collectable. In this case,
        // selecting confirm with the keyboard will bring the cursor up already.
        // The end result is that it needs fewer presses of the confirm key
        // than otherwise.
        //
        // At the end of this sequence the cursor should have selected the recipe
        // again and be on the Synthesize button.
        if task.collectable {
            ui::wait(1.0);
            ui::press_confirm(handle);
            // Give the UI a moment
            ui::wait(3.0);
            ui::press_confirm(handle)
        } else {
            ui::wait(4.0);
            ui::press_confirm(handle);
        }
    }
}

fn execute_actions(handle: &xiv::XivHandle, actions: &[macros::Action]) {
    for action in actions {
        // Each character has a 20ms wait and the shortest action string
        // we can make (observe or reclaim) is 240 ms, along with 50ms
        // from send_action. That reduces how much time is needed to wait
        // here for the GCD to finish. Although macros always wait in 2 or
        // 3 second periods, the actual wait period is 2.0 and 2.5 seconds,
        // so that's adjusted here.
        send_action(handle, &action.name);
        if action.wait == 2 {
            ui::wait(1.7);
        } else {
            ui::wait(2.2);
        };
    }
}

fn send_action(handle: &xiv::XivHandle, action: &str) {
    log::debug!("action(`{}`)", action);
    ui::press_enter(handle);
    ui::send_string(handle, &format!("/ac \"{}\"", action));
    ui::press_enter(handle);
}

fn change_gearset(handle: &xiv::XivHandle, gearset: u64) {
    log::debug!("gearset({})", gearset);
    println!("changing to gearset {}", gearset);
    ui::press_enter(handle);
    ui::send_string(handle, &format!("/gearset change {}", gearset));
    ui::press_enter(handle);
}

fn toggle_collectable(handle: &xiv::XivHandle) {
    send_action(handle, &"collectable synthesis");
}

pub fn aaction(handle: &xiv::XivHandle, verb: &str, action: &str) {
    ui::press_enter(handle);
    if verb == "clear" {
        ui::send_string(handle, "/aaction clear");
    } else {
        ui::send_string(handle, &format!("/aaction \"{}\" {}", action, verb));
    }
    ui::press_enter(handle);
}

pub fn aaction_clear(handle: &xiv::XivHandle) {
    aaction(handle, "clear", "");
    ui::wait(1.0)
}

pub fn aaction_add(handle: &xiv::XivHandle, action: &str) {
    aaction(handle, "on", action)
}

pub fn aaction_remove(handle: &xiv::XivHandle, action: &str) {
    aaction(handle, "off", action)
}
