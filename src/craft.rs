use crate::cross;
use crate::macros;
use crate::task::Task;
use crate::ui;
// Runs through the set of tasks
// TODO: make it actually run more than one task
pub fn craft_items(tasks: &[Task]) {
    for task in tasks {
        clear_windows();
        if task.collectable {
            toggle_collectable();
        }

        // Bring up the crafting window itself and give it time to appear
        ui::open_craft_window();
        ui::wait_secs(1);

        // Navigate to the correct recipe based on the index provided
        select_recipe(&task);

        // Time to craft the items
        execute_task(&task);

        clear_windows();
        if task.collectable {
            toggle_collectable();
        }
    }
}

fn clear_windows() {
    println!("clearing window...");
    // Hitting escape closes one window each. 10 is excessive, but conservative
    for _ in 0..10 {
        ui::escape();
    }

    // Cancelling twice will close the System menu if it is open
    ui::cancel();
    ui::cancel();
    ui::wait_secs(1);
}

fn select_recipe(task: &Task) {
    println!("selecting recipe...");
    // Loop backward through the UI 9 times to ensure we hit the text box
    // no matter what crafting class we are. The text input boxes are strangely
    // modal so that if we select them at any point they will hold on to focus
    // for characters.
    for _ in 0..9 {
        ui::move_backward();
    }

    ui::confirm();
    send_string(&task.item_name);
    ui::wait_ms(200);
    ui::enter();

    // It takse up to a second for results to populate
    ui::wait_secs(1);

    // Navigate to the offset we need
    for _ in 0..task.index {
        ui::cursor_down();
    }

    // Select the recipe to get to components / craft
    ui::confirm();
}

fn execute_task(task: &Task) {
    for task_index in 0..task.count {
        // On subsequent crafts we need to navigate from the recipe to the Synthesize
        // button again.
        if task_index > 0 {
            ui::confirm();
        }
        // Hit the Synthesize button and wait for the window to pop up
        ui::confirm();
        ui::confirm();
        ui::confirm();
        ui::wait_secs(2);

        execute_actions(&task.actions);

        // If the item is collectable we'll have an additional dialog
        if task.collectable {
            ui::wait_secs(1);
            ui::confirm();
        }

        // Wait to get back to the crafting window
        if task.collectable {
            ui::wait_secs(1);
            ui::confirm();
            ui::wait_secs(4);
        } else {
            ui::wait_secs(4);
        };
    }
}

fn execute_actions(actions: &Vec<macros::Action>) {
    for action in actions {
        // Each character has a 20ms wait and the shortest action string
        // we can make (observe or reclaim) is 240 ms, along with 50ms
        // from send_action. That reduces how much time is needed to wait
        // here for the GCD to finish. Although macros always wait in 2 or
        // 3 second periods, the actual wait period is 2.0 and 2.5 seconds,
        // so that's adjusted here.
        send_action(&action.name);
        if action.wait == 2 {
            ui::wait_ms(1700);
        } else {
            ui::wait_ms(2200);
        };
    }
}

pub fn send_string(s: &str) {
    for c in s.chars() {
        ui::send_char(c);
    }
}

pub fn send_action(action: &str) {
    ui::enter();
    send_string(&format!("/ac \"{}\"\n", action));
    ui::wait_ms(50);
    ui::enter();
}

fn toggle_collectable() {
    send_action(&"collectable synthesis");
}
