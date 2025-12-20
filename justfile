list:
    just --list

build-juvycore:
    just juvycore build

build-ios:
    just juvycore build-ios

install: install-rust install-typeshare

install-typeshare:
    echo "==> Installing Typeshare..."
    cargo install typeshare-cli@1.13.4

install-rust:
    #!/usr/bin/env bash
    set -euo pipefail
    if command -v rustup &>/dev/null; then
        echo "Rust already installed ($(rustc --version))"
    else
        echo "==> Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        echo "==> Rust installed successfully"
        echo "Run 'source ~/.cargo/env' or restart your terminal"
    fi

mod juvycore