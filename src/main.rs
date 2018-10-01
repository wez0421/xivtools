//use std::collections::HashMap;
mod xiv_macro;
use config;
use failure::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Item(s) being crafted are collectible
    #[structopt(short = "c", long = "collectible")]
    collectible: bool,
    /// Print verbose information during execution
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    /// Number of items to craft
    #[structopt(short = "n", long = "count", default_value = "1")]
    count: u32,
    /// Path to the file containing the XIV macro to use
    #[structopt(name = "macro file", parse(from_os_str))]
    macro_file: PathBuf,
    /// Path to the config file.
    #[structopt(long = "config", default_value = "config.toml")]
    config_file: String,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    println!("opt {:?}", opt);

    // Grab and parse the config file. Errors are all especially fatal so
    // let them bubble up if they occur.
    let mut config = config::Config::default();
    config.merge(config::File::with_name(&opt.config_file))?;
    let macro_contents = xiv_macro::parse_file(opt.macro_file)?;
    let system_keys = config.get_table("system_keybinds")?;
    let crafting_keys = config.get_table("crafting_keybinds")?;
    println!("System Keys:");
    for (key, value) in &system_keys {
        println!("\t{} = '{}'", key, value);
    }
    println!("Crafting Keys:");
    for (key, value) in &crafting_keys {
        println!("\t{} = '{}'", key, value);
    }
    println!("Macro to Run");
    for entry in macro_contents {
        println!("\t{}", entry);
    }
    Ok(())
}
