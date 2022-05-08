# Jumpy

Jumpy is a tool that allows to quickly jump to one of the directory you've visited in the past.

It is heavily inspired by [Zoxide](https://github.com/ajeetdsouza/zoxide/) but is more lightweight and a lot faster.

In its current version it is mostly intended for my personal use, if I find to work well enough I'll improve the documentation and add new features.

## Setup

For ZSH shells:

```shell
function z() {
    local result=$(jumpy query "$1" --after "$PWD")

    if [[ -n $result ]]; then
        export __JUMPY_DONT_REGISTER=1
        cd "$result"
        export __JUMPY_DONT_REGISTER=0
    fi
}

function jumpy_handler() {
    if (( $__JUMPY_DONT_REGISTER )); then
        return
    fi

    emulate -L zsh
    jumpy add "$PWD"
}

chpwd_functions=(${chpwd_functions[@]} "jumpy_handler")
```

This will allow Jumpy to register each change of directory to add them to its database.

To perform a query and jump to it, just use `z <query>`.

## Usage

```shell
# Get the most relevant directory from a query
jumpy query <terms>

# Add a new directory to the database, or increment its score
jumpy add <terms>

# List all registered directories, sorted by score
jumpy list

# Clear the database
jumpy clear
```
