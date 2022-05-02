mod action;
mod config;
mod craft;
mod gui;
mod lists;
mod macros;
mod recipe;
mod rpc;
mod task;

use anyhow::{Error, Result};
use rpc::{Request, Response, Worker};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "talan", about = "A FFXIV Crafting helper")]
struct Opts {
    /// Path to the macro file
    #[structopt(short = "m", long = "macros", default_value = "macros.toml")]
    macro_path: PathBuf,
    /// Path to the config file
    #[structopt(short = "c", long = "config", default_value = config::DEFAULT_CONFIG_FILE)]
    config_path: PathBuf,
    /// Enable log levels (use multiple -v for more logging)
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u64,
}

fn parse_arguments() -> Result<(PathBuf, PathBuf), Error> {
    let args = Opts::from_args();
    env_logger::Builder::from_default_env()
        .filter(
            Some("talan"),
            match args.verbose {
                0 => log::LevelFilter::Info,
                1 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            },
        )
        .init();
    Ok((args.config_path, args.macro_path))
}

fn main() -> Result<(), Error> {
    let (config_path, macros_path) = parse_arguments()?;
    log::debug!("config file: {:?}", config_path);
    log::debug!("macros file: {:?}", macros_path);
    let mut cfg = config::get_config(Some(&config_path));
    let (client_tx, worker_rx): (Sender<Request>, Receiver<Request>) = channel();
    let (worker_tx, client_rx): (Sender<Response>, Receiver<Response>) = channel();
    thread::spawn(move || Worker::new(worker_rx, worker_tx).worker_thread());

    let mut gui = gui::Gui::new(config_path, macros_path, &client_tx, &client_rx);
    gui.start(&mut cfg);

    println!("exiting...");
    Ok(())
}
