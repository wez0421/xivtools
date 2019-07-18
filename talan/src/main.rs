mod config;
mod gui;
mod macros;
mod task;

use env_logger;
use failure::Error;
use task::Task;

fn main() -> Result<(), Error> {
    env_logger::init();

    let _handle = xiv::init()?;
    // Read here, but can be updated by a UI impl.
    let mut cfg = config::read_config();
    // Any UI implementation will populate this vec.
    let mut tasks: Vec<Task> = Vec::new();
    // These are only read at startup.
    let macros = macros::get_macro_list()?;
    log::info!("Scanning macros:");
    for m in &macros {
        log::info!("\t{}", m.name);
    }

    match gui::start(&mut cfg, &mut tasks, &macros)? {
        true => println!("tasks: {:#?}", tasks),
        false => println!("exiting..."),
    }

    Ok(())
}
