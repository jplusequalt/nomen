use anyhow::{self, bail};

use crate::app::{Create, Run};
use crate::utils;
use home;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

impl Run for Create {
    fn run(&self) -> anyhow::Result<()> {
        let mut command = self.command.clone();
        let args = utils::split_arguments(&mut command);

        if *args.get(0).unwrap() == "cd" {
            let mut p = PathBuf::from(*args.get(1).unwrap_or_else(|| &""));

            p = utils::strip_home_prefix(&mut p).unwrap();

            if !p.exists() {
                let err_msg = format!("the path '{}' does not exist", p.display());
                bail!(err_msg);
            }
        } else {
            let status = Command::new(args.get(0).unwrap())
                .args(args[1..].to_vec())
                .output()
                .expect("Error!");
            if !status.status.success() {
                let err_msg = format!(
                    "the command '{}' with the following arguments '{}' is not valid",
                    args.get(0).unwrap(),
                    args[1..].iter().fold(String::new(), |acc, &arg| acc + arg)
                );
                bail!(err_msg);
            }
        }

        let mut file = match utils::get_alias_file() {
            Some(p) => OpenOptions::new().append(true).open(p).unwrap(),
            None => {
                let mut s = String::new();

                println!(".bash_aliases does not exist! Do you want to create it? (y/n)");

                io::stdin().read_line(&mut s)?;

                if s.to_ascii_lowercase().trim() == &String::from("y") {
                    println!("Creating .bash_aliases ...");
                    let mut h = home::home_dir().unwrap();
                    h.push(".bash_aliases");
                    OpenOptions::new().write(true).create(true).open(h).unwrap()
                } else {
                    bail!("exiting ...");
                }
            }
        };

        writeln!(file, "{}", format!("alias {}='{}'", self.name, command))?;

        println!("Added alias {}", self.name);

        Ok(())
    }
}
