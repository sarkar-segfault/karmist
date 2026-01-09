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
    pub cmd: Option<Command>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Add(AddCommand),
    List(ListCommand),
    Update(UpdateCommand),
    Remove(RemoveCommand),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "add", description = "add a task")]
pub struct AddCommand {
    #[argh(positional)]
    pub id: Option<String>,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "list", description = "list all tasks")]
pub struct ListCommand {}

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
