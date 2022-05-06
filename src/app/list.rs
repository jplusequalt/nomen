use anyhow;

use crate::app::{List, Run};

impl Run for List {
    fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
