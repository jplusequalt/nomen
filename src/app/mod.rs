mod _app;
mod create;
mod list;
mod remove;
mod update;

use anyhow::Result;

pub use crate::app::_app::*;

pub trait Run {
    fn run(&self) -> Result<()>;
}

impl Run for App {
    fn run(&self) -> Result<()> {
        match self {
            App::Create(create) => create.run(),
            App::Update(update) => update.run(),
            App::Remove(remove) => remove.run(),
            App::List => List.run(),
        }
    }
}
