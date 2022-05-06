mod app;
mod utils;

use std::process::exit;

use crate::app::{App, Run};
use clap::Parser;

fn main() {
    match App::parse().run() {
        Ok(_) => exit(0),
        Err(e) => eprintln!("nomen: {}", e),
    };
}
