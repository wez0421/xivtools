use std::collections::HashMap;
use std::env;
extern crate config;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("invalid argument count!");
        println!("usage: {} [path/to/settings/file] [path to macro file] [craft count]", args[0]);
        return;
    }

    let settings_file = &args[1];
    let _macro_file = &args[2];
    let _run_count = &args[3];
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(settings_file)).expect("Failed to read config");

    println!("{:?}\n",
             settings.try_into::<HashMap<String, String>>().unwrap());


    println!("{:?}", args);
}
