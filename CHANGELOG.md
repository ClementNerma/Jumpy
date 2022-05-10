# CHANGELOG

## v0.3.2 (2022-05-10)

* **FIX:** `--top` flag for `inc` command being ignored if directory already registered

## v0.3.1 (2022-05-10)

* **NEW:** `inc <path> --top` subcommand to assign the maximum score to the provided directory

## v0.3.0 (2022-05-10)

* **NEW:** `export` subcommand
* **NEW:** `path` subcommand to print the path to the database file
* **NEW:** `path --lossily` to print to the path to the database file lossily (if it contains invalid UTF-8 characters)

* **INTERNAL:** Make database file sorted

* **FIX:** Don't flush index to disk when no change were made

## v0.2.0 (2022-05-10)

* **BREAKING:** `add <path>` subcommand doesn't increment registered directories anymore

* **NEW:** `inc <path>` replaces the previous `add <path>` subcommand's behaviour
* **NEW:** `query <path> --checked` to find the first existing directory (previous ones will be removed from database)
* **NEW:** `optimize` subcommand to optimize database

* **PERF:** Switch to a `HashMap` instead of using a `BTreeMap`

* **STABILITY:** Automatically check when database changes need to be flushed to the disk

## v0.1.1 (2022-05-08)

* **NEW:** Added `del <path>` action to remove a registered directory from the database

* **FIX:** Correctly flush index to disk after writings

## v0.1.0 (2022-05-08)

Initial version of Jumpy!

* Support for adding directories to database with `add <path>`
* Support for querying database with `query <path>`
* Support for listing all registered directories with `list`
* Support for clearing database with `clear`
* Support for custom index file path with `--index-file`
* Support for ranked query results
* Support for results cycling depending on CWD
