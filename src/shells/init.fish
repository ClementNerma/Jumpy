
function __jumpy_reg --on-variable PWD --description 'Register directory changes with Jumpy'
    if set -q __JUMPY_DONT_REGISTER
        return
    end

    jumpy inc "$PWD"
end

function z -a query
    set result $(jumpy query "$query" --checked --after "$PWD")

    if test -n "$result"
        set __JUMPY_DONT_REGISTER
        cd "$result"
        set -e __JUMPY_DONT_REGISTER
    end
end
