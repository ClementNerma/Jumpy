#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![forbid(unused_allocation)]

mod cmd;
mod index;

use std::{fs, io::stdout};

use clap::{CommandFactory, Parser};

use crate::{
    cmd::*,
    index::{Index, IndexEntry},
};

static INDEX_FILENAME: &str = "jumpy.db";

fn fail(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}

fn main() {
    let cmd = Command::parse();

    let index_file = cmd.index_file.unwrap_or_else(|| {
        dirs::config_dir()
            .expect("Failed to get configuration directory!")
            .join(INDEX_FILENAME)
    });

    let (mut index, source) = if index_file.exists() {
        let content = fs::read_to_string(&index_file)
            .unwrap_or_else(|e| fail(&format!("Failed to read index file: {e}")));

        (
            Index::decode(&content)
                .unwrap_or_else(|e| fail(&format!("Failed to decode index: {e}"))),
            content,
        )
    } else {
        (Index::new(), String::new())
    };

    match cmd.action {
        Action::Add(Add { path }) => {
            index
                .add(path)
                .unwrap_or_else(|e| fail(&format!("Failed to add directory: {e}")));
        }

        Action::Inc(Inc { path, top }) => {
            index
                .inc(path, top)
                .unwrap_or_else(|e| fail(&format!("Failed to increment directory: {e}")));
        }

        Action::Query(Query {
            query,
            after,
            checked,
        }) => {
            if query.is_empty() {
                fail("Please provide a query to search from.");
            }

            let result = if checked {
                index.query_checked(&query, after.as_deref())
            } else {
                index
                    .query_unchecked(&query, after.as_deref())
                    .map(str::to_string)
            };

            match result {
                Some(result) => println!("{result}"),
                None => fail("No result found"),
            }
        }

        Action::List(List { scores }) => {
            let mut entries = index.iter().collect::<Vec<_>>();

            if scores {
                entries.sort_by_key(|entry| entry.score);
                entries.reverse();
            } else {
                entries.sort_by_key(|entry| entry.path);
            }

            let longest_score = entries
                .iter()
                .map(|entry| entry.score)
                .max()
                .map(|score| score.to_string().len())
                .unwrap_or(0);

            for IndexEntry { path, score } in entries {
                if scores {
                    println!("{score:>longest_score$} {path}");
                } else {
                    println!("{path}");
                }
            }
        }

        Action::Del(Del { path }) => {
            index.remove(&path).unwrap();
        }

        Action::Clear(Clear {}) => {
            index.clear();
        }

        Action::Cleanup(Cleanup {}) => index.cleanup(),

        Action::Export(Export {}) => index.export(),

        Action::Path(Path { lossily }) => match index_file.to_str() {
            Some(lossless) => println!("{}", lossless),
            None => {
                if lossily {
                    println!("{}", index_file.to_string_lossy())
                } else {
                    fail("Path to index file contains invalid UTF-8 characters. Use --lossily to print it nonetheless.");
                }
            }
        },

        Action::Completions(Completions { for_shell }) => {
            use clap_complete::*;

            let shell = match for_shell {
                CompletionShellName::Bash => Shell::Bash,
                CompletionShellName::Zsh => Shell::Zsh,
                CompletionShellName::Fish => Shell::Fish,
                CompletionShellName::Elvish => Shell::Elvish,
                CompletionShellName::PowerShell => Shell::PowerShell,
            };

            let cmd = &mut Command::command();

            aot::generate(shell, cmd, cmd.get_name().to_string(), &mut stdout());
        }
    }

    let updated = index.encode();

    if updated != source {
        fs::write(&index_file, updated)
            .unwrap_or_else(|e| fail(&format!("Failed to write index file: {e}")))
    }
}
