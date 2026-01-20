#!/usr/bin/env bash
set -e

# Default locations
USER_BIN="$HOME/.local/bin/rusty-notes"
SYSTEM_BIN="/usr/local/bin/rusty-notes"

echo "Starting rusty-notes uninstall..."

# Remove binary
if [[ -f "$USER_BIN" ]]; then
    echo "Removing user binary: $USER_BIN"
    rm -f "$USER_BIN"
elif [[ -f "$SYSTEM_BIN" ]]; then
    echo "Removing system binary: $SYSTEM_BIN (requires sudo)"
    sudo rm -f "$SYSTEM_BIN"
else
    echo "No rusty-notes binary found."
fi

# Remove configuration directory (~/.notes)
read -p "Do you want to remove your notes directory (~/.notes)? [y/N]: " CONFIRM
CONFIRM=${CONFIRM,,} # lowercase
if [[ "$CONFIRM" == "y" || "$CONFIRM" == "yes" ]]; then
    NOTES_DIR="$HOME/.notes"
    if [[ -d "$NOTES_DIR" ]]; then
        echo "Deleting $NOTES_DIR ..."
        rm -rf "$NOTES_DIR"
        echo "Notes removed."
    else
        echo "No notes directory found."
    fi
else
    echo "Notes directory preserved."
fi

echo "rusty-notes uninstalled successfully."
