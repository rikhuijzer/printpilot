shebang := "/usr/bin/env bash"

alias s := serve-site

default:
    just --list

generate-site:
    #!{{shebang}}

    set -euo pipefail

    mkdir -p _public/

    cargo build --bin=core --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/debug/core.wasm public/
    cargo run --bin=site --profile=dev -- generate site

serve-site:
    #!{{shebang}}

    set -euo pipefail

    mkdir -p _public/

    # Using lomirus/live-server.
    live-server --port 8080 _public/ & SERVER_PID=$!

    # Trap to kill the server when the process exits. Without this, the port
    # will remain in use and a new server will fail to start.
    trap "kill $SERVER_PID" EXIT

    # Run cargo in the foreground.
    cargo build --bin=core --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/debug/core.wasm _public/
    cargo watch -x "run --bin=site --profile=dev -- generate site"
