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

    // let opt = Opt::from_args();
    // // Can this become map err?
    // let handle = xiv::init();

    // // Grab and parse the config file. Errors are all especially fatal so
    // // let them bubble up if they occur.
    // let macro_contents =
    //     macros::parse_file(opt.macro_file).map_err(|e| format!("error parsing macro: `{}`", e));

    // let item = garland::fetch_item_info(&opt.item_name)?;
    // log::info!("item information: {}", item);
    // let tasks = vec![task::Task {
    //     item: item,
    //     index: opt.recipe_index,
    //     count: opt.count,
    //     actions: macro_contents.unwrap(),
    //     gearset: opt.gearset,
    //     collectable: opt.collectable,
    // }]
    let mut cfg = config::read_config();
    let macros = macros::get_macro_list()?;
    gui::start(&mut cfg, &macros)?;
    //craft::craft_items(&handle, &tasks);
    Ok(())
}
