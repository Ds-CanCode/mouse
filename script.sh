
#!/usr/bin/env bash
set -e

GO_INfre="/goinfre/$USER"

echo "==> Setting up Rust in $GO_INfre"

# Installation directories
export CARGO_HOME="$GO_INfre/cargo"
export RUSTUP_HOME="$GO_INfre/rustup"

mkdir -p "$CARGO_HOME" "$RUSTUP_HOME"

# Install Rust if rustup is missing
if [ ! -x "$CARGO_HOME/bin/rustup" ]; then
    echo "==> Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | env CARGO_HOME="$CARGO_HOME" RUSTUP_HOME="$RUSTUP_HOME" sh -s -- -y
else
    echo "==> Rustup already installed."
fi

# Configure PATH
export PATH="$CARGO_HOME/bin:$PATH"

# Configure Zsh permanently
ZSHRC="$HOME/.zshrc"

for line in \
    'export CARGO_HOME="/goinfre/$USER/cargo"' \
    'export RUSTUP_HOME="/goinfre/$USER/rustup"' \
    'export PATH="$CARGO_HOME/bin:$PATH"'
do
    grep -Fqx "$line" "$ZSHRC" 2>/dev/null || echo "$line" >> "$ZSHRC"
done

# Install stable toolchain
rustup toolchain install stable
rustup default stable

echo "==> Rust installation complete!"
rustc --version
cargo --version

