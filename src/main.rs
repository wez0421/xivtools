//use std::collections::HashMap;
mod xiv_macro;
mod xiv_ui;
use self::xiv_ui::HWND;
use std::ptr::null_mut;
use failure::Error;
use std::path::PathBuf;
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
    /// Path to the file containing the XIV macro to use
    #[structopt(name = "macro file", parse(from_os_str))]
    macro_file: PathBuf,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    // Grab and parse the config file. Errors are all especially fatal so
    // let them bubble up if they occur.
    let macro_contents = xiv_macro::parse_file(opt.macro_file)?;
    println!("Macro to Run:");

    let mut window: xiv_ui::HWND = null_mut();
    xiv_ui::find_xiv_window(&mut window);
    xiv_ui::reset_ui(&window);
    xiv_ui::reset_action_keys(&window);

    if opt.collectable {
        xiv_ui::toggle_collectable_status(&window);
    }

    xiv_ui::bring_craft_window(&window, &opt.item, opt.offset);
    for i in 0..opt.count {
        xiv_ui::craft_item(&window, &macro_contents, opt.collectable);  
    }
    
    xiv_ui::reset_ui(&window);

    if opt.collectable {
        xiv_ui::toggle_collectable_status(&window);
    }
    Ok(())
}
