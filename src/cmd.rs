use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum, ValueHint};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub index_file: Option<PathBuf>,

    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    #[clap(
        about = "Add a new directory if not yet registered",
        long_about = "Does nothing if the directory is already registered"
    )]
    Add(Add),

    #[clap(
        about = "Increment a registered directory's score or add it to the database",
        long_about = "Adds the directory to the database if it is not registered yet"
    )]
    Inc(Inc),

    #[clap(about = "Find the most relevant directory for the provided query")]
    Query(Query),

    #[clap(about = "List all registered directories")]
    List(List),

    #[clap(about = "Delete a registered directory from the database")]
    Del(Del),

    #[clap(about = "Cleanup the database to remove deleted directories")]
    Cleanup(Cleanup),

    #[clap(about = "Clear the database")]
    Clear(Clear),

    #[clap(about = "Output the entire database (plain text)")]
    Export(Export),

    #[clap(about = "Get the path of the index file")]
    Path(Path),

    #[clap(about = "Generate completions for a given shell")]
    Completions(Completions),
}

#[derive(Args)]
pub struct Add {
    #[clap(value_hint = ValueHint::DirPath)]
    pub path: String,
}

#[derive(Args)]
pub struct Inc {
    #[clap(value_hint = ValueHint::DirPath)]
    pub path: String,

    #[clap(long, help = "Give the maximum score to this directory")]
    pub top: bool,
}

#[derive(Args)]
pub struct Query {
    #[clap()]
    pub query: String,

    #[clap(short, long, value_hint = ValueHint::DirPath)]
    pub after: Option<String>,

    #[clap(short, long)]
    pub checked: bool,
}

#[derive(Args)]
pub struct List {
    #[clap(
        long,
        help = "Only display paths (sort order will be alphabetic instead of score-based)"
    )]
    pub just_paths: bool,
}

#[derive(Args)]
pub struct Del {
    #[clap(value_hint = ValueHint::DirPath)]
    pub path: String,
}

#[derive(Args)]
pub struct Cleanup {}

#[derive(Args)]
pub struct Clear {}

#[derive(Args)]
pub struct Export {}

#[derive(Args)]
pub struct Path {
    #[clap(
        short,
        long,
        help = "If the path contains invalid UTF-8 characters, don't fail and print it lossily instead"
    )]
    pub lossily: bool,
}

#[derive(Args)]
pub struct Completions {
    #[clap(help = "Shell to generate completions for")]
    pub for_shell: CompletionShellName,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum CompletionShellName {
    Bash,
    Zsh,
    Fish,
    Elvish,
    PowerShell,
}
