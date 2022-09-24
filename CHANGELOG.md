# CHANGELOG

## v0.3.6 (2022-09-24)

* **Fix::** Don't register root path (`/`) as it doesn't have a filename to match against

## v0.3.5 (2022-09-19)

* **BREAKING:** Renamed the `optimize` action to `cleanup`

## v0.3.4 (2022-09-19)

* **Fix:** Remove entries from the database with `optimize` when a directory became a file

## v0.3.3 (2022-05-10)

* **Fix:** Score ranking was reversed in `query` and `list`

## v0.3.2 (2022-05-10)

* **Fix:** `--top` flag for `inc` command being ignored if directory already registered

## v0.3.1 (2022-05-10)

* **New:** `inc <path> --top` subcommand to assign the maximum score to the provided directory

## v0.3.0 (2022-05-10)

* **New:** `export` subcommand
* **New:** `path` subcommand to print the path to the database file
* **New:** `path --lossily` to print to the path to the database file lossily (if it contains invalid UTF-8 characters)

* **Internal:** Make database file sorted

* **Fix:** Don't flush index to disk when no change were made

## v0.2.0 (2022-05-10)

* **BREAKING:** `add <path>` subcommand doesn't increment registered directories anymore

* **New:** `inc <path>` replaces the previous `add <path>` subcommand's behaviour
* **New:** `query <path> --checked` to find the first existing directory (previous ones will be removed from database)
* **New:** `optimize` subcommand to optimize database

* **Perf:** Switch to a `HashMap` instead of using a `BTreeMap`

* **Stability:** Automatically check when database changes need to be flushed to the disk

## v0.1.1 (2022-05-08)

* **New:** Added `del <path>` action to remove a registered directory from the database

* **Fix:** Correctly flush index to disk after writings

## v0.1.0 (2022-05-08)

Initial version of Jumpy!

* Support for adding directories to database with `add <path>`
* Support for querying database with `query <path>`
* Support for listing all registered directories with `list`
* Support for clearing database with `clear`
* Support for custom index file path with `--index-file`
* Support for ranked query results
* Support for results cycling depending on CWD
