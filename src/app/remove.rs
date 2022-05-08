use anyhow::{bail, Result};

use crate::app::{Remove, Run};
use crate::utils;
use crate::App;
use clap::CommandFactory;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

impl Run for Remove {
    fn run(&self) -> Result<()> {
        let p = match utils::get_alias_file() {
            Some(p) => p,
            None => {
                let mut cmd = App::command();
                cmd.error(
                    clap::ErrorKind::Io,
                    ".bash_aliases was not found! try running: nomen create --name <NAME> --command <COMMAND> to create a new .bash_aliases file"
                ).exit();
            }
        };

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&p)
            .expect("Error opening .bash_aliases!");

        let mut num_lines = 0;

        let lines = BufReader::new(file)
            .lines()
            .map(|line| {
                num_lines += 1;
                line.unwrap()
            })
            .filter(|line| !line.starts_with(format!("alias {}=", self.name).as_str()))
            .collect::<Vec<String>>();

        if num_lines > lines.len() {
            std::fs::write(&p, lines.join("\n")).expect("Error writing to .bash_aliases!");
            println!("Removed alias: {}", self.name);
        } else {
            bail!("no alias named {}", self.name);
        }

        Ok(())
    }
}
