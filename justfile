shebang := "/usr/bin/env bash"

alias b := build
alias s := serve

default:
    just --list

# Build the website.
build:
    #!{{shebang}}

    set -eux pipefail

    trunk build --release

# Serve the website locally with live reload.
serve:
    #!{{shebang}}

    set -euo pipefail

    # Cooldown is necessary for JS files in static somehow.
    trunk serve --enable-cooldown

