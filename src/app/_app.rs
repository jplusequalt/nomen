use clap::{AppSettings, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub enum App {
    /// Create new aliases
    Create(Create),

    /// Update existing aliases
    Update(Update),

    /// Remove aliases
    Remove(Remove),

    /// List aliases
    List,
}

#[derive(Parser)]
pub struct Create {
    /// The name of the new alias
    #[clap(short, long, required = true)]
    pub name: String,
    /// The command to be aliased
    #[clap(short, long, required = true)]
    pub command: String,
}

#[derive(Parser)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Update {
    /// The current name of the alias you wish to update
    #[clap(short, long, required = true)]
    pub name: String,

    /// The new name of the alias (conflicts with new_command)
    #[clap(long = "new_name")]
    pub new_name: Option<String>,

    /// The new command to be aliased (conflicts with new_name)
    #[clap(long = "new_cmd")]
    pub new_command: Option<String>,
}

#[derive(Parser)]
pub struct Remove {
    #[clap(short, required = true)]
    pub name: String,
}

#[derive(Parser)]
pub struct List;
