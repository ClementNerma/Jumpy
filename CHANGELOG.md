# CHANGELOG

## v0.1.4 (2022-05-10)

* **NEW:** `optimize` subcommand to optimize database
* **PERF:** Switch to a `HashMap` instead of using a `BTreeMap`

## v0.1.3 (2022-05-10)

* **NEW:** `query <path> --checked` to find the first existing directory (previous ones will be removed from database)
* **STABILITY:** Automatically check when database changes need to be flushed to the disk

## v0.1.2 (2022-05-10)

* **BREAKING:** `add <path>` subcommand doesn't increment registered directories anymore
* **NEW:** `inc <path>` replaces the previous `add <path>` subcommand's behaviour

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
