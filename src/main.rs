mod node;
mod tree;
mod misc;
mod app;
mod state;
mod system;
mod resource;
mod component;

use serde_json;

use std::env;
use std::fs;

use amethyst;
use app::*;

fn main() {
    let mut conf_path = None;

    for arg in env::args().skip(1) {
        if arg.starts_with("-c") {
            conf_path = Some(arg.split_at(14).1.to_string());
        }
    }

    let conf = if let Some(path) = conf_path {
        path
    } else {
        "./config.json".to_string()
    };
    println!("Loading configuration from {}", conf);

    let conf_file = fs::File::open(conf).expect("Failed to read config file");
    let config = serde_json::from_reader(conf_file).expect("Failed to parse config file");

    run(&config);
}


