use anyhow::{bail, Result};

use crate::app::{Run, Update};
use crate::utils;
use crate::App;
use clap::{CommandFactory, ErrorKind};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

impl Run for Update {
    fn run(&self) -> Result<()> {
        let p = match utils::get_alias_file() {
            Some(p) => p,
            None => {
                bail!(".bash_aliases was not found! try running: 
                        nomen create --name <NAME> --command <COMMAND> to create a new .bash_aliases file");
            }
        };

        let mut file = OpenOptions::new().read(true).write(true).open(&p).unwrap();

        let mut s = String::new();
        file.read_to_string(&mut s)?;
        drop(file);

        if let None = s.find(format!("alias {}=", self.name).as_str()) {
            bail!("no alias named {} was found", self.name);
        }

        let mut new_buf = Vec::new();

        let mut which = false;

        for line in s.lines() {
            if line.starts_with(format!("alias {}", self.name).as_str()) {
                match (&self.new_name, &self.new_command) {
                    (Some(new_name), None) => {
                        which = true;
                        new_buf.push(line.replace(
                            format!("alias {}", self.name).as_str(),
                            format!("alias {}", new_name).as_str(),
                        ));
                    }
                    (None, Some(new_command)) => {
                        let offset = self.name.len() + 8;

                        new_buf.push(line.replace(
                            &line[offset..(line.len() - 1)],
                            format!("{}", new_command).as_str(),
                        ));
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
                new_buf.push(line.to_string());
            }
        }

        let mut new_file = File::create(&p)?;

        for item in new_buf {
            writeln!(new_file, "{}", item)?;
        }

        if which {
            println!(
                "Updated alias '{}' name to '{}'",
                self.name,
                self.new_name.as_ref().unwrap()
            );
        } else {
            println!(
                "Updated alias '{}' command: now '{}'",
                self.name,
                self.new_command.as_ref().unwrap()
            );
        }

        Ok(())
    }
}
