use anyhow::{anyhow, Error, Result};
use chrono::{Local, NaiveDateTime};
use std::collections::HashSet;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};
use structopt::StructOpt;
use xiv::{ui, classjob};
mod retainer;

#[derive(Debug, StructOpt)]
#[structopt(name = "ventures", about = "A FFXIV venture automation helper")]
struct Opts {
    /// Use slower menu navigation for slower or laggier systems
    #[structopt(short = "s", long = "slow")]
    use_slow_navigation: bool,

    /// Enable log levels.
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u64,

    /// The name of a retainer to ignore
    #[structopt(short = "i", long = "ignore")]
    ignore: Vec<String>,
}

#[derive(Debug)]
struct Retainer {
    id: u64,
    period: Duration,
    next: Instant,
}

fn parse_arguments() -> Result<(xiv::XivHandle, Vec<String>), Error> {
    let args = Opts::from_args();
    env_logger::Builder::from_default_env()
        .filter(
            Some("ventures"),
            match args.verbose {
                1 => log::LevelFilter::Debug,
                2 => log::LevelFilter::Trace,
                _ => log::LevelFilter::Info,
            },
        )
        .init();

    let mut h = xiv::init()?;
    h.use_slow_navigation = args.use_slow_navigation;

    Ok((h, args.ignore))
}

fn main() -> Result<(), Error> {
    let proc = process::Process::new("ffxiv_dx11.exe")?;
    let (hnd, ignore_vec) = parse_arguments()?;
    let ignore_set: HashSet<String> = ignore_vec.into_iter().collect();

    println!("ignore_set: {:#?}", ignore_set);
    let mut retainers = retainer::Retainers::new(&proc, retainer::OFFSET);
    retainers.read()?;
    if retainers.total_retainers == 0 {
        return Err(anyhow!(
            "No retainers found! Try opening a retainer bell at least once after logging in."
        ));
    }

    let mut cnt = 0;
    for r in retainers.retainer.iter() {
        if r.available {
        println!("[{} {}] {}", r.level, classjob::ClassJob::from(r.classjob), r.name());
            cnt += 1;
        }
    }
    let cnt: usize = retainers
        .retainer
        .iter()
        .fold(0, |acc, &r| acc + r.available as usize);

    loop {
        let mut menu_open = false;
        retainers.read()?;

        for rdx in 0..cnt {
            if ignore_set.contains(&retainers.retainer[rdx].name().to_string()) {
                continue;
            }

            if retainers.retainer[rdx].venture_id == 0 {
                continue;
            }

            let pos = retainers
                .display_order
                .iter()
                .position(|&r_id| r_id == rdx as u8)
                .unwrap();
            let now = Local::now().timestamp();
            let done = retainers.retainer[rdx].venture_complete;
            if done <= now as u32 {
                let mut updated = false;
                let mut retries = 3;
                // If for some reason we don't update the venture (Lag, UI state, etc) then clear the window and try this retainer again.
                while !updated && retries > 0 {
                    if !menu_open {
                        open_retainer_menu(hnd);
                        menu_open = true;
                    }

                    log::info!("re-assigning {}'s venture", retainers.retainer[rdx].name());
                    reassign_venture(hnd, pos);
                    retainers.read()?;

                    if retainers.retainer[rdx].venture_complete != done {
                        updated = true;
                    } else {
                        log::error!(
                            "{}'s venture did not update. Retrying.",
                            retainers.retainer[rdx].name()
                        );
                        menu_open = false;
                        retries -= 1;
                    }
                }
            }
        }
        if menu_open {
            ui::press_cancel(hnd);
        }
        thread::sleep(Duration::from_secs(60));
        print!(".");
        io::stdout().flush().unwrap();
        // At this point any ventures that were complete have been re-assigned
        // so we can update our table cache and see who is next.
        // let next_retainer = retainers.retainer[0..cnt]
        //     .iter()
        //     .min_by_key(|&r| {
        //         if r.venture_id != 0 && !ignore_set.contains(&r.name().to_string()) {
        //             r.venture_complete
        //         } else {
        //             now
        //         }
        //     })
        //     .unwrap();
        // log::info!(
        //     "Next: {} @ {} utc",
        //     next_retainer.name(),
        //     NaiveDateTime::from_timestamp(next_retainer.venture_complete as i64, 0)
        //         .format("%D %H:%M:%S")
        // );

        // let sleep_time = Duration::from_secs(
        //     (next_retainer.venture_complete as i64 - Local::now().timestamp()) as u64,
        // );

        // // Clear the menu before sleep.
        // ui::press_cancel(hnd);
        // if
        // thread::sleep(sleep_time);
    }
}

fn clear_retainer_window(hnd: xiv::XivHandle) {
    xiv::ui::clear_window(hnd);
    ui::press_escape(hnd);
    ui::wait(1.0);
    ui::press_escape(hnd);
    ui::wait(1.0);
    ui::press_cancel(hnd);
    ui::wait(1.0);
    ui::press_cancel(hnd);
    ui::wait(1.0);
}

fn open_retainer_menu(hnd: xiv::XivHandle) {
    log::debug!("open_retainer_menu");
    // This will close the game menu if open and exit the retainer window if
    // it was open from a previous run.
    ui::press_escape(hnd);
    ui::press_escape(hnd);
    ui::press_cancel(hnd);
    ui::press_cancel(hnd);
    ui::wait(2.0);

    // The reason the menu is opened twice is because we want to clear out any
    // mouse actions the UI registered that would lead to us not having the input
    // cursor up when the retainer menu opens.
    ui::target_nearest_npc(hnd);
    ui::wait(1.0);
    ui::press_confirm(hnd);
    ui::wait(2.0);
    ui::press_cancel(hnd);
    ui::press_cancel(hnd);
    ui::target_nearest_npc(hnd);
    ui::wait(1.0);
    ui::press_confirm(hnd);
    ui::wait(2.0);
}

// General usability rules
// 1. Wait 1 second after moving around in a menu
// 2. Wait 2 seconds after pressing a button for UI changes / Feo Ul / Retainer dialog.
fn reassign_venture(hnd: xiv::XivHandle, r_id: usize) {
    log::debug!("reassign_venture(r_id: {})", r_id);
    for _ in 0..r_id {
        ui::cursor_down(hnd);
    }
    ui::wait(1.0);
    ui::press_confirm(hnd);
    ui::wait(2.0);
    ui::press_confirm(hnd);
    ui::wait(2.0);
    // Move down to Assign Venture / View Venture Progress
    for _ in 0..5 {
        ui::cursor_down(hnd);
    }
    ui::wait(1.0);
    // Select the menu option
    ui::press_confirm(hnd);
    ui::wait(2.0);
    // Move left to 'Reassign'
    ui::cursor_left(hnd);
    ui::wait(1.0);
    // Confirm 'Reassign'
    ui::press_confirm(hnd);
    ui::wait(2.0);
    // Move left to 'Assign' in the venture window that comes up
    ui::cursor_left(hnd);
    ui::wait(1.0);
    // Confirm 'Assign'
    ui::press_confirm(hnd);
    ui::wait(2.0);
    // Confirm the message from the retainer about the venture
    ui::press_confirm(hnd);
    ui::wait(2.0);
    // Escape out of the specific retainer's menu
    ui::press_cancel(hnd);
    ui::wait(2.0);
    // Say goodbye to the retainer
    ui::press_confirm(hnd);
    ui::wait(2.0);
}
