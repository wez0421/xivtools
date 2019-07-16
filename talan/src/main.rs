mod config;
mod gui;
mod macros;
mod task;

use clap::{App, Arg};
use env_logger;
use failure::Error;

const VERSION: &str = "1.0";

fn main() -> Result<(), Error> {
    env_logger::init();

    let matches = App::new("Ventures")
        .version(VERSION)
        .arg(
            Arg::with_name("gui")
                .short("g")
                .help("Run with a graphical interface."),
        )
        .get_matches();

    let _handle = xiv::init()?;
    let mut cfg = config::read_config();
    let macros = macros::get_macro_list()?;

    if matches.occurrences_of("gui") > 0 {
        gui::start(&mut cfg, &macros)?;
    }
    Ok(())
}
