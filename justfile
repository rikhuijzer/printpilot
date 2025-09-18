shebang := "/usr/bin/env bash"

alias s := serve

default:
    just --list

build:
    #!{{shebang}}

    set -euo pipefail

    trunk build

serve:
    #!{{shebang}}

    set -euo pipefail

    trunk serve
