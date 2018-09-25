//use std::collections::HashMap;
mod xiv_macro;
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
}

fn main() -> Result<(), String> {
    let _opt = Opt::from_args();

    Ok(())
}
