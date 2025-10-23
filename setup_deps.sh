#!/bin/bash
# Setup script for korg_nano_kontrol_2 dependency
# This script clones the dependency if it doesn't exist

DEPS_DIR="../korg_nano_kontrol_2"
REPO_URL="https://github.com/JoshuaBatty/korg_nano_kontrol_2.git"

if [ -d "$DEPS_DIR" ]; then
    echo "✓ korg_nano_kontrol_2 dependency already exists at $DEPS_DIR"
else
    echo "Cloning korg_nano_kontrol_2 dependency..."
    if git clone "$REPO_URL" "$DEPS_DIR"; then
        echo "✓ Successfully cloned korg_nano_kontrol_2"
    else
        echo "✗ Failed to clone korg_nano_kontrol_2"
        exit 1
    fi
fi

echo ""
echo "Dependencies ready! You can now run:"
echo "  cargo build"
echo "  cargo run"
