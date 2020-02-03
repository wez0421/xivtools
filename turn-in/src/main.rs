use anyhow::{Error, Result};
use env_logger;
use structopt;
use structopt::StructOpt;
use xiv;
use xiv::ui;

#[derive(Debug, StructOpt)]
#[structopt(name = "turn-in", about = "A FFXIV venture automation helper")]
struct Opts {
    /// Use slower menu navigation for slower or laggier systems
    #[structopt(short = "s", long = "slow")]
    use_slow_navigation: bool,

    /// The position of the item being turned in in the npc's list.
    /// Starts at 1 for the top item.
    #[structopt(short = "e", long = "entry", default_value = "1")]
    entry: u64,

    /// The number of items to turn in.
    #[structopt(short = "c", long = "count", default_value = "1")]
    count: u64,

    /// Enable log levels.
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u64,
}

fn parse_arguments() -> Result<(u64, u64, xiv::XivHandle), Error> {
    let args = Opts::from_args();
    env_logger::Builder::from_default_env()
        .filter(
            Some("turn-in"),
            match args.verbose {
                1 => log::LevelFilter::Debug,
                2 => log::LevelFilter::Trace,
                _ => log::LevelFilter::Info,
            },
        )
        .init();

    // Parse a mix of ranges specified by X-Y or separated by commas X,Y,Z
    let mut h = xiv::init()?;
    h.use_slow_navigation = args.use_slow_navigation;

    Ok((args.count, args.entry, h))
}

fn main() -> Result<(), Error> {
    let (count, entry, hnd) = parse_arguments()?;

    // Bring up the cursor and move to the appropriate entry
    ui::press_confirm(hnd);
    for _ in 1..entry {
        ui::cursor_down(hnd);
    }

    // Turn in the items
    for _ in 0..count {
        ui::press_confirm(hnd);
        ui::wait(0.5);
        ui::press_subcommands(hnd);
        ui::wait(0.5);
        ui::press_confirm(hnd);
        ui::press_confirm(hnd);
    }

    Ok(())
}
