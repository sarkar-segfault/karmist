use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "todoist, but for gigachads")]
pub struct Options {
    #[argh(
        option,
        short = 'd',
        long = "database",
        default = "format!(\"{:?}/.karmist.json\", std::env::home_dir().unwrap_or_else(|| fatal!(\"failed to acquire home directory\")))",
        description = "the task database"
    )]
    pub db: String,

    #[argh(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Create(CreateCommand),
    Read(ReadCommand),
    Update(UpdateCommand),
    Remove(RemoveCommand),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "create", description = "create a task")]
pub struct CreateCommand {
    #[argh(positional)]
    pub id: Option<String>,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "read", description = "read one, or all tasks")]
pub struct ReadCommand {
    #[argh(positional)]
    pub id: Option<String>,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "update", description = "update a task")]
pub struct UpdateCommand {
    #[argh(positional)]
    pub id: Option<String>,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "remove", description = "remove a task")]
pub struct RemoveCommand {
    #[argh(positional)]
    pub id: Option<String>,
}
