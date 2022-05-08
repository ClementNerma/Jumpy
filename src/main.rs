mod cmd;
mod index;

use clap::Parser;

static INDEX_FILENAME: &str = "jumpy.db";

fn fail(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}

fn main() {
    use cmd::*;
    use index::Index;
    use std::fs;

    let cmd = Command::parse();

    let index_file = cmd.index_file.unwrap_or_else(|| {
        dirs::config_dir()
            .expect("Failed to get configuration directory!")
            .join(INDEX_FILENAME)
    });

    let mut index = if index_file.exists() {
        let content = fs::read_to_string(&index_file)
            .unwrap_or_else(|e| fail(&format!("Failed to read index file: {e}")));

        Index::decode(&content).unwrap_or_else(|e| fail(&format!("Failed to decode index: {e}")))
    } else {
        Index::new()
    };

    let flush_index = |index: Index| fs::write(&index_file, index.encode()).unwrap_or_else(
        |e| fail(&format!("Failed to write index file: {e}"))
    );

    match cmd.action {
        Action::Add(Add { path }) => {
            index
                .add(path)
                .unwrap_or_else(|e| fail(&format!("Failed to add directory: {e}")));

            flush_index(index);
        }

        Action::Query(Query { query, after }) => {
            if query.is_empty() {
                fail("Please provide a query to search from.");
            }

            let result = index.query(&query, after.as_deref());

            match result {
                Some(result) => println!("{result}"),
                None => fail("No result found"),
            }
        }

        Action::List(List {}) => {
            for dir in index.list() {
                println!("{dir}");
            }
        }

        Action::Del(Del { path }) => {
            index.remove(&path).unwrap();

            flush_index(index);
        },

        Action::Clear(Clear {}) => {
            index.clear();

            flush_index(index);
        }
    }
}
