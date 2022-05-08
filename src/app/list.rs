use anyhow;

use crate::app::{List, Run};
use crate::utils;
use crate::App;
use clap::{CommandFactory, ErrorKind};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

impl Run for List {
    fn run(&self) -> anyhow::Result<()> {
        let p = match utils::get_alias_file() {
            Some(p) => p,
            None => {
                let mut cmd = App::command();
                cmd.error(
                    ErrorKind::Io,
                    ".bash_aliases was not found! try running: nomen create --name <NAME> --command <COMMAND> to create a new .bash_aliases file"
                ).exit();
            }
        };

        let file = OpenOptions::new()
            .read(true)
            .open(&p)
            .expect("Error opening .bash_aliases!");

        println!("Printing aliases ...");

        BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .for_each(|line| println!("\t- {}", line));

        Ok(())
    }
}
