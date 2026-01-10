use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "todoist, but for gigachads")]
pub struct Options {
    #[argh(
        option,
        short = 'd',
        long = "database",
        default = "\".karmist.json\".to_string()",
        description = "the task database"
    )]
    pub db: String,

    #[argh(subcommand)]
    pub cmd: Command,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Create(CreateCommand),
    Read(ReadCommand),
    Update(UpdateCommand),
    Delete(DeleteCommand),
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
#[argh(subcommand, name = "delete", description = "delete one, or all task")]
pub struct DeleteCommand {
    #[argh(positional)]
    pub id: Option<String>,
}
