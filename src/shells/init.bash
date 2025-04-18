# NOTE: This is a hack as Bash doesn't support current directory watchers
function cd {
    builtin cd "$@" || return
    jumpy inc "$PWD"
}

function z {
    if [[ -z $1 ]]; then
        echo "ERROR: Please provide a query."
        return 1
    fi

    local result=$(jumpy query "$1" --checked --after "$PWD")

    if [[ -n $result ]]; then
        builtin cd "$result"
    fi
}
