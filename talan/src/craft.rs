//use crate::role_actions::RoleActions;
use crate::action::Action;
use crate::config::Options;
use crate::macros::Macro;
use crate::task;
use log;
use std::thread::sleep;
use std::time::{Duration, Instant};
use xiv::ui;

// Milliseconds to pad the GCD to account for latency
const GCD_PADDING: u64 = 250;

// Craft all the configured tasks and update the client by way of |status_callback|.
pub fn craft_items<'a, S, C>(
    mut handle: xiv::XivHandle,
    options: &'a Options,
    macros: &[Macro],
    tasks: &[task::Task],
    mut status_fn: S,
    mut continue_fn: C,
) where
    S: FnMut(&[task::Status]),
    C: FnMut() -> bool,
{
    // Initialize the crafting status and send an initialize slice
    // so the UI knows what to start rendering.
    let mut status: Vec<task::Status> = tasks.iter().map(task::Status::from).collect();
    status_fn(&status[..]);

    handle.use_slow_navigation = options.use_slow_dialog_navigation;
    if options.should_clear_window_on_craft {
        // Get the UI into a state we can trust it, and pray the user doesn't touch it.
        ui::clear_window(handle);
    }

    // Clear role actions before we iterate tasks so the game state
    // and role action state will be in sync.
    let mut job: u32 = 256;
    for (i, task) in tasks.iter().enumerate() {
        log::trace!("Task: {:?}", task);
        let task_job: usize = task.recipe.job as usize;

        if options.gear[task_job] == 0 {
            panic!(
                "No gear set is configured for {}, aborting tasks!",
                xiv::JOBS[task_job]
            );
        }

        // Swap our job if necessary. It may have been used in the previous task.
        if job != task.recipe.job {
            log::trace!("changing job to {}.", xiv::JOBS[task_job]);
            change_gearset(handle, options.gear[task_job]);
            // If we don't wait here we might bring the window up before
            // the job has changed, leading to the wrong class seeding the
            // window's mode.
            ui::wait(1.0);

            job = task.recipe.job;
        } else {
            log::trace!("already {}, no need to change job.", xiv::JOBS[task_job]);
        }

        // Navigate to the correct recipe based on the index provided
        select_recipe(handle, &task);
        select_materials(handle, &task);
        for task_index in 1..=task.quantity {
            log::info!(
                "crafting {} {}/{}",
                task.recipe.name,
                task_index,
                task.quantity
            );
            // Time to craft the items
            if !continue_fn()
                || !execute_task(
                    handle,
                    &macros[task.macro_id as usize].actions[..],
                    &mut continue_fn,
                )
            {
                log::info!("Received stop order");
                return;
            }
            status[i].finished += 1;
            status_fn(&status[..]);
            // Check if we received a message to stop from the main thread.
            ui::wait(2.0);
        }

        ui::press_escape(handle);
        ui::wait(2.0);
    }
}

pub fn open_craft_window(handle: xiv::XivHandle) {
    ui::send_key(handle, 'N' as i32);
    ui::wait(1.0);
}

// Selects the appropriate recipe then leaves the cursor on the Synthesize
// button, ready for material selection.
pub fn select_recipe(handle: xiv::XivHandle, task: &task::Task) {
    // Bring up the crafting window itself and give it time to appear
    open_craft_window(handle);
    log::info!("selecting recipe...");
    // The crafting window always starts with the current job selected and if we press
    // |BACK| 1 more time than the job's index then we will end up at the search box.
    for _ in 0..=task.recipe.job + 1 {
        ui::cursor_backward(handle);
    }
    ui::press_confirm(handle);
    ui::wait(1.0);
    ui::send_string(handle, &task.recipe.name);
    ui::press_enter(handle);
    ui::wait(1.0);
    // Navigate to the offset we need
    for _ in 0..task.recipe.index {
        ui::cursor_down(handle);
    }

    // Select the recipe to get to components / sythen
    ui::press_confirm(handle);
}

pub fn select_any_materials(handle: xiv::XivHandle, task: &task::Task) {
    // Up to the icon for the bottom material
    ui::cursor_up(handle);
    // Right to the NQ column
    ui::cursor_right(handle);
    // Right to the HQ column
    ui::cursor_right(handle);

    // The cursor should be on the quantity field of the bottom item now
    // We move through the ingredients backwards because we start at the bottom of t
    for (i, material) in task.recipe.mats.iter().rev().enumerate() {
        log::debug!("{}x {}", material.count, material.name);
        for _ in 0..material.count {
            ui::press_confirm(handle)
        }
        // Don't move up if we've made it back to the top of the ingredients
        if i != task.recipe.mats.len() - 1 {
            ui::cursor_up(handle);
        }
    }
    ui::cursor_left(handle);
    for material in &task.recipe.mats {
        for _ in 0..material.count {
            ui::press_confirm(handle)
        }
        ui::cursor_down(handle);
    }
}

pub fn select_materials(handle: xiv::XivHandle, task: &task::Task) {
    if !task.specify_materials {
        return select_any_materials(handle, task);
    }

    let mut hq_mats = task.mat_quality.iter().fold(0, |acc, &mat| acc + mat.hq);
    // If there are no HQ mats we can fast path this by just
    // starting the synthesis.
    if hq_mats == 0 {
        return;
    }

    // Up to the icon for the bottom material
    ui::cursor_up(handle);
    // Right to the NQ column
    ui::cursor_right(handle);
    // Right to the HQ column
    ui::cursor_right(handle);

    // Move up the HQ column and increase the HQ count per the task
    // values. Once there are none left we can shortcut back to the
    // confirm button.
    for (i, mq) in task.mat_quality.iter().rev().enumerate() {
        for _ in 0..mq.hq {
            ui::press_confirm(handle);
        }

        hq_mats -= mq.hq;
        if hq_mats > 0 {
            ui::cursor_up(handle);
        } else {
            for _ in 0..=i {
                ui::cursor_down(handle);
            }
            break;
        }
    }
}

fn execute_task<C>(handle: xiv::XivHandle, actions: &[&'static Action], continue_fn: &mut C) -> bool
where
    C: FnMut() -> bool,
{
    // If we're at the start of a task we will already have the Synthesize button
    // selected with the pointer.
    ui::press_confirm(handle);

    // The first action is one second off so we start typing while the
    // crafting window is coming up.
    let mut next_action = Instant::now() + Duration::from_secs(2);
    let mut prev_action = next_action;
    for action in actions {
        if !continue_fn() {
            return false;
        }

        ui::press_enter(handle);
        ui::send_string(handle, &format!("/ac \"{}\"", &action.name));
        // At this point the action is queued in the text buffer, so we can
        // wait the GCD duration based on the last action we sent.
        let mut now = Instant::now();
        if now < next_action {
            let delta = next_action - now;
            log::trace!("sleeping {:?}", delta);
            sleep(delta);
        }
        ui::press_enter(handle);
        now = Instant::now();
        log::debug!("action: {} ({:?})", action.name, now - prev_action);
        prev_action = now;
        next_action = now + Duration::from_millis(action.wait_ms + GCD_PADDING);
    }

    if !continue_fn() {
        return false;
    }

    // Wait for the last GCD to finish
    sleep(next_action - Instant::now());

    // At the end of this sequence the cursor should have selected the recipe
    // again and be on the Synthesize button.
    ui::wait(3.0);
    ui::press_confirm(handle);
    true
}

pub fn change_gearset(handle: xiv::XivHandle, gearset: i32) {
    log::info!("changing to gearset {}", gearset);
    ui::press_enter(handle);
    ui::send_string(handle, &format!("/gearset change {}", gearset));
    ui::wait(0.5);
    ui::press_enter(handle);
}
