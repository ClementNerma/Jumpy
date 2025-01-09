use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum, ValueHint};

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
    Add {
        #[clap(value_hint = ValueHint::DirPath)]
        path: String,
    },

    #[clap(
        about = "Increment a registered directory's score or add it to the database",
        long_about = "Adds the directory to the database if it is not registered yet"
    )]
    Inc {
        #[clap(value_hint = ValueHint::DirPath)]
        path: String,

        #[clap(long, help = "Give the maximum score to this directory")]
        top: bool,
    },

    #[clap(about = "Find the most relevant directory for the provided query")]
    Query {
        #[clap()]
        query: String,

        #[clap(short, long, value_hint = ValueHint::DirPath)]
        after: Option<String>,

        #[clap(short, long)]
        checked: bool,
    },

    #[clap(about = "List all registered directories")]
    List {
        #[clap(short, long, help = "Display scores and sort directories by them")]
        scores: bool,
    },

    #[clap(about = "Delete a registered directory from the database")]
    Del {
        #[clap(value_hint = ValueHint::DirPath)]
        path: String,
    },

    #[clap(about = "Cleanup the database to remove deleted directories")]
    Cleanup {},

    #[clap(about = "Clear the database")]
    Clear {},

    #[clap(about = "Output the entire database (plain text)")]
    Export {},

    #[clap(about = "Get the path of the index file")]
    Path {
        #[clap(
            short,
            long,
            help = "If the path contains invalid UTF-8 characters, don't fail and print it lossily instead"
        )]
        lossily: bool,
    },

    #[clap(about = "Generate completions for a given shell")]
    Completions {
        #[clap(help = "Shell to generate completions for")]
        for_shell: CompletionShellName,
    },
}
#[derive(Clone, Copy, ValueEnum)]
pub enum CompletionShellName {
    Bash,
    Zsh,
    Fish,
    Elvish,
    PowerShell,
}
