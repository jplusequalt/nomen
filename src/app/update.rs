use anyhow::{bail, Result};

use crate::app::{Run, Update};
use crate::utils;
use crate::App;
use clap::{CommandFactory, ErrorKind};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

impl Run for Update {
    fn run(&self) -> Result<()> {
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

        let file = OpenOptions::new().read(true).write(true).open(&p).unwrap();

        let mut alias = String::new();

        let mut lines = BufReader::new(&file)
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| {
                if line.starts_with(format!("alias {}=", self.name).as_str()) {
                    alias = line.clone();
                    return false;
                }
                true
            })
            .collect::<Vec<String>>();

        drop(file);

        let mut msg = String::new();

        if !alias.is_empty() {
            match (&self.new_name, &self.new_command) {
                (Some(new_name), None) => {
                    lines.push(alias.replace(
                        format!("alias {}", self.name).as_str(),
                        format!("alias {}", new_name).as_str(),
                    ));

                    msg.push_str(
                        format!(
                            "Updated alias '{}' name to '{}'",
                            self.name,
                            self.new_name.as_ref().unwrap()
                        )
                        .as_str(),
                    );
                }
                (None, Some(new_command)) => {
                    let offset = self.name.len() + 8;

                    lines.push(alias.replace(
                        &alias[offset..(alias.len() - 1)],
                        format!("{}", new_command).as_str(),
                    ));

                    msg.push_str(
                        format!(
                            "Updated alias '{}' command: now '{}'",
                            self.name,
                            self.new_command.as_ref().unwrap()
                        )
                        .as_str(),
                    );
                }
                (None, None) => {
                    let mut cmd = App::command();
                    cmd.error(
                            ErrorKind::ArgumentNotFound,
                            "update requires exactly one argument of either --new_name or --new_command",
                        )
                        .exit();
                }
                _ => {
                    let mut cmd = App::command();
                    cmd.error(
                        ErrorKind::ArgumentNotFound,
                        "conflicting arguments for update: \
                                     use either --new_name or --new_command, but not both",
                    )
                    .exit();
                }
            }
        } else {
            bail!("no alias named {}", self.name);
        }

        let mut new_file = File::create(&p)?;

        for item in lines {
            writeln!(new_file, "{}", item)?;
        }

        println!("{}", msg);

        Ok(())
    }
}
