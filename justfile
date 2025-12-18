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

build-gum:
    just gum build

mod gum
