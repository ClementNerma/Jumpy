use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(short, long)]
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

    #[clap(about = "Optimize the database")]
    Optimize(Optimize),

    #[clap(about = "Clear the database")]
    Clear(Clear),
}

#[derive(Args)]
pub struct Add {
    #[clap()]
    pub path: String,
}

#[derive(Args)]
pub struct Inc {
    #[clap()]
    pub path: String,
}

#[derive(Args)]
pub struct Query {
    #[clap()]
    pub query: String,

    #[clap(short, long)]
    pub after: Option<String>,

    #[clap(short, long)]
    pub checked: bool,
}

#[derive(Args)]
pub struct List {}

#[derive(Args)]
pub struct Del {
    #[clap()]
    pub path: String,
}

#[derive(Args)]
pub struct Optimize {}

#[derive(Args)]
pub struct Clear {}
