use anyhow::Result;

use crate::app::{Remove, Run};

impl Run for Remove {
    fn run(&self) -> Result<()> {
        println!("Running remove!");
        Ok(())
    }
}
