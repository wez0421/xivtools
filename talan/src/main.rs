mod config;
mod gui;
mod macros;
mod task;

use env_logger;
use failure::Error;

//mod craft;
//mod role_actions;
//mod recipe;

fn main() -> Result<(), Error> {
    env_logger::init();

    // // Grab and parse the config file. Errors are all especially fatal so
    // // let them bubble up if they occur.
    // let macro_contents =
    //     macros::parse_file(opt.macro_file).map_err(|e| format!("error parsing macro: `{}`", e));

    let _handle = xiv::init()?;
    let mut cfg = config::read_config();
    let macros = macros::get_macro_list()?;
    gui::start(&mut cfg, &macros)?;
    //craft::craft_items(&handle, &tasks);
    Ok(())
}
