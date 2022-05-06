use clap::{ArgGroup, Parser};

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
#[clap(group(
    ArgGroup::new("update")
        .required(true)
        .args(&["new_name", "new_command"])
))]
pub struct Update {
    /// The current name of the alias you wish to update
    #[clap(short, long, required = true)]
    pub name: String,

    /// The new name of the alias (conflicts with new_command)
    #[clap(long = "new_name")]
    pub new_name: String,

    /// The new command to be aliased (conflicts with new_name)
    #[clap(long = "new_cmd")]
    pub new_command: String,
}

#[derive(Parser)]
pub struct Remove {
    name: String,
}

#[derive(Parser)]
pub struct List;
