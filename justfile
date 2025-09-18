shebang := "/usr/bin/env bash"

alias s := serve

default:
    just --list

serve:
    #!{{shebang}}

    set -euo pipefail

    trunk serve
