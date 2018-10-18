//use std::collections::HashMap;
mod macros;
mod ui;
use self::ui::HWND;
use failure::Error;
use std::path::PathBuf;
use std::ptr::null_mut;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Item(s) will be crafted as collectable
    #[structopt(long = "collectable")]
    collectable: bool,
    /// Print verbose information during execution
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    /// Name of the iterm to craft
    #[structopt(short = "i", long = "item-name", default_value = "")]
    item: String,
    /// Number of items to craft
    #[structopt(short = "c", long = "count", default_value = "1")]
    count: u32,
    #[structopt(short = "o", long = "offset", default_value = "0")]
    offset: u32,
    /// Path to the file containing the XIV macros to use
    #[structopt(name = "macros file", parse(from_os_str))]
    macro_file: PathBuf,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    // Grab and parse the config file. Errors are all especially fatal so
    // let them bubble up if they occur.
    let macros_contents = macros::parse_file(opt.macro_file)?;
    println!("Macro to Run:");

    let mut window: ui::HWND = ui::platform_default_hwnd();
    ui::find_xiv_window(&mut window);
    ui::reset_ui(&window);
    ui::reset_action_keys(&window);

    if opt.collectable {
        ui::toggle_collectable_status(&window);
    }

    ui::bring_craft_window(&window, &opt.item, opt.offset);
    for i in 0..opt.count {
        ui::craft_item(&window, &macros_contents, opt.collectable);
    }

    ui::reset_ui(&window);

    if opt.collectable {
        ui::toggle_collectable_status(&window);
    }
    Ok(())
}
