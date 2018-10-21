use crate::task::Task;
use crate::ui;
use std::time;

// Runs through the set of tasks
// TODO: make it actually run more than one task
pub fn craft_items(tasks: &Vec<Task>) {
    for task in tasks {
        // First we need to clear the UI
        // TODO: can this be done with cancel instead?
        for _ in 0..5 {
            ui::escape();
            ui::wait_ms(100);
        }

        if task.collectable {
            toggle_collectable();
        }

        // Bring up the crafting window itself and give it time to appear
        ui::open_craft_window();

        // Loop backward through the UI 8 times to ensure we hit the text box
        // no matter what crafting class we are. The text input boxes are strangely
        // modal so that if we select them at any point they will hold on to focus
        // for characters.
        for _ in 0..8 {
            ui::move_backward();
        }

        // Navigate to the correct recipe based on the index provided
        select_recipe(&task);

        // Time to craft the items
        execute_task(&task);

        // Clear out the UI again 
        for _ in 0..5 {
            ui::escape();
            ui::wait_ms(100);
        }

        if task.collectable {
            toggle_collectable();
        }
    }
}

fn select_recipe(task: &Task) {
    // The search dialog should be selected, so send our string and search

    send_string(&task.item_name);
    ui::wait_ms(200);
    ui::enter();

    // It takse up to a second for results to populate
    ui::wait_secs(1);

    // First confirm will get the pointer in the recipe list
    ui::confirm();

    // Navigate to the offset we need
    for _ in 0..task.index {
        ui::cursor_down();
    }

    // Select the recipe to get to components / craft
    ui::confirm();
}

fn execute_task(task: &Task) {
    for _ in 0..task.count {
        // Hit the craft button and wait for the window to pop up
        ui::confirm();
        ui::wait_secs(2);

        for action in &task.actions {
            send_action(&action.name, action.wait);
        }

        // If the item is collectable we'll have an additional dialog
        if task.collectable {
            ui::wait_secs(1);
            ui::confirm();
        }

        // Wait to get back to the crafting window
        ui::wait_secs(1);
    }
}

pub fn send_string(s: &str) {
    for c in s.chars() {
        ui::send_char(c);
    }
}

pub fn send_action(action: &str, wait: u64) {
    // Activate the chat box
    ui::enter();
    ui::wait_ms(100);
    // Send /ac "<action>"
    send_string("/ac \"");
    send_string(action);
    send_string("\"");
    // Wait for text to fill out
    ui::wait_ms(100);
    // Commit the action
    ui::enter();
    // Wait for the action's gcd
    ui::wait_secs(wait);
}

fn toggle_collectable() {
    send_action("collectable synthesis", 2);
}