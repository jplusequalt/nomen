use anyhow::{bail, Result};

use crate::app::{Run, Update};
use crate::utils;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom};

impl Run for Update {
    fn run(&self) -> Result<()> {
        let mut file = match utils::get_alias_file() {
            Some(p) => OpenOptions::new().read(true).write(true).open(p).unwrap(),
            None => {
                bail!(".bash_aliases was not found! try running: 
                        nomen create --name <NAME> --command <COMMAND> to create a new .bash_aliases file");
            }
        };

        let mut s = String::new();
        file.read_to_string(&mut s)?;

        let (new_name, new_cmd) = (&self.new_name, &self.new_command);

        match (new_name, new_cmd) {
            (String, _) => {
                for mut line in s.lines() {
                    if line.starts_with(format!("alias {}:", self.name).as_str()) {
                        line = line
                            .strip_prefix(format!("alias {}", self.name).as_str())
                            .unwrap();
                    }
                }
            }
            (_, String) => {
                for mut line in s.lines() {
                    if line.starts_with(format!("alias {}:", self.name).as_str()) {
                        let pos = line.find("=").unwrap();
                        let mut s_line = String::from(line);
                        s_line = s_line[..pos].to_string() + new_cmd;
                        line = s_line.as_str();
                    }
                }
            }
        }

        Ok(())
    }
}
