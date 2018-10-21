mod craft;
mod macros;
mod task;
mod ui;

use failure::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::craft::craft_items;
use crate::task::{ Task, Jobs };

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Item(s) will be crafted as collectable
    #[structopt(long = "collectable")]
    collectable: bool,
    /// Print verbose information during execution
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// For recipes which have multiple search results this offset is used to
    /// determine the specific recipe to use. Offsets start at 0 for the first
    /// recipe in search results and increment by one for each recipe down.
    #[structopt(short = "i", long = "index", default_value = "0")]
    recipe_index: u32,
    /// Path to the file containing the XIV macros to use
    #[structopt(name = "macro file", parse(from_os_str))]
    macro_file: PathBuf,
    /// Name of the item to craft
    #[structopt(name = "item name")]
    item: String,
    /// Number of items to craft
    #[structopt(default_value = "1")]
    count: u32,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    // Grab and parse the config file. Errors are all especially fatal so
    // let them bubble up if they occur.
    let macro_contents = macros::parse_file(opt.macro_file)?;
    let tasks = vec![ Task {
        item_name: "cloud pearl".to_string(),
        index: 8,
        count: 1,
        collectable: true,
        actions: macro_contents,
        job: Jobs::CUL,
    }];
    craft::craft_items(&tasks);
    Ok(())
}
