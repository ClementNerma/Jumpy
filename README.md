# Jumpy

Jumpy is a tool that allows to quickly jump to one of the directory you've visited in the past.

It is heavily inspired by [Zoxide](https://github.com/ajeetdsouza/zoxide/) but is more lightweight and a lot faster.

In its current version it is mostly intended for my personal use, if I find to work well enough I'll improve the documentation and add new features.

Updates can be found in the [releases](https://github.com/ClementNerma/Jumpy/releases).

## Performance

On a Ryzen 7900 (running on a single core), it takes about 4 seconds to decode a 500 MB index file with 10 million registered directories, and 2 seconds to traverse it entirely to find the very last entry.

On a small and more realistic example, with 1 thousand directories, it takes about 250 µs to decode the 50 KB index file and 250 µs to traverse it to find the last entry.

## Setup

```shell
# ZSH
eval "$(jumpy completions zsh)"

# Fish
jumpy completions fish | source
```

This will allow Jumpy to register each change of directory to add them to its database.

To perform a query and jump to it, just use `z <query>`.

## Usage

```shell
# [With shell integration] Jumpy to the first directory matching the query
z <terms>

# Get the most relevant directory from a query
jumpy query <terms>

# Add a new directory to the database, or increment its score
jumpy add <terms>

# List all registered directories, sorted by score
jumpy list

# Clear the database
jumpy clear
```
