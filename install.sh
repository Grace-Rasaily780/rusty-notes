#!/usr/bin/env bash
set -e

git clone https://github.com/Grace-Rasaily780/rusty-notes ~/rusty-notes
cd ~/rusty-notes

# Build the release binary
echo "Building rusty-notes..."
cargo build --release

# Create local bin if missing
mkdir -p "$HOME/.local/bin"

# Copy binary
cp target/release/rusty-notes "$HOME/.local/bin/rn"

# Make executable
chmod +x "$HOME/.local/bin/rn"

echo "Installed rusty-notes to $HOME/.local/bin"
echo "Make sure $HOME/.local/bin is in your PATH."

rm -rf ~/rusty-notes

