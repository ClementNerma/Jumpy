export __JUMPY_DONT_REGISTER=0

function jumpy_handler() {
    if (( $__JUMPY_DONT_REGISTER )); then
        return
    fi

    emulate -L zsh
    jumpy inc "$PWD"
}

function z() {
    [[ -z $1 ]] && {{ echo "ERROR: Please provide a query."; return 1 }}

    local result=$(jumpy query "$1" --checked --after "$PWD")

    if [[ -n $result ]]; then
        export __JUMPY_DONT_REGISTER=1
        cd "$result"
        export __JUMPY_DONT_REGISTER=0
    fi
}

chpwd_functions=(${chpwd_functions[@]} "jumpy_handler")
