use anyhow::{anyhow, Error, Result};
use chrono::{Local, NaiveDateTime};
use std::thread;
use std::time::{Duration, Instant};
use structopt::StructOpt;
use xiv::ui;
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
}

#[derive(Debug)]
struct Retainer {
    id: u64,
    period: Duration,
    next: Instant,
}

fn parse_arguments() -> Result<xiv::XivHandle, Error> {
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

    Ok(h)
}

fn main() -> Result<(), Error> {
    let proc = process::Process::new("ffxiv_dx11.exe")?;
    let hnd = parse_arguments()?;

    let mut retainer_table = retainer::Retainers::new(&proc, retainer::OFFSET);
    retainer_table.read()?;
    if retainer_table.total_retainers == 0 {
        return Err(anyhow!(
            "No retainers found! Try opening a retainer bell at least once after logging in."
        ));
    }

    let cnt: usize = retainer_table
        .retainer
        .iter()
        .fold(0, |acc, &r| acc + r.available as usize);

    loop {
        let mut menu_open = false;
        retainer_table.read()?;

        for (r_index, &retainer) in retainer_table.retainer[0..cnt].iter().enumerate() {
            if retainer.venture_id == 0 {
                continue;
            }

            if retainer.venture_complete as i64 <= Local::now().timestamp() {
                // The retainer position may have changed between venture runs
                // so it's calculated here. The retainer will always be in the
                // display list as far as I can tell.
                let r_pos = match retainer_table
                    .display_order
                    .iter()
                    .position(|&r_id| r_id == r_index as u8)
                {
                    Some(v) => v,
                    None => {
                        return Err(anyhow!(
                            "Couldn't find {} in the display order list. This might be a bug?",
                            retainer.name()
                        ))
                    }
                };

                if !menu_open {
                    open_retainer_menu(hnd);
                    menu_open = true;
                }

                log::info!("re-assigning {}'s venture", retainer.name());
                reassign_venture(hnd, r_pos as u64);
            }
        }

        // At this point any ventures that were complete have been re-assigned
        // so we can update our table cache and see who is next.
        retainer_table.read()?;
        let next_retainer = retainer_table.retainer[0..cnt]
            .iter()
            .min_by_key(|&r| r.venture_complete)
            .unwrap();
        log::info!(
            "Next: {} @ {} utc",
            next_retainer.name(),
            NaiveDateTime::from_timestamp(next_retainer.venture_complete as i64, 0)
                .format("%H:%M:%S")
        );

        let sleep_time = Duration::from_secs(
            (next_retainer.venture_complete as i64 - Local::now().timestamp()) as u64,
        );

        // Clear the menu before sleep.
        ui::press_cancel(hnd);
        thread::sleep(sleep_time);
    }
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
fn reassign_venture(hnd: xiv::XivHandle, r_id: u64) {
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
