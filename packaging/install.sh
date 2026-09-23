#!/bin/bash
set -e

# CN File Organizer Installation Script

if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (use sudo)"
  exit 1
fi

echo "Installing CN File Organizer..."

# Copy desktop entry
cp packaging/cn-organizer.desktop /usr/share/applications/
chmod 644 /usr/share/applications/cn-organizer.desktop

# Check if binary is compiled (or built via Tauri DEB package)
if [ -f "crates/cn-cli/target/release/organize" ]; then
    cp crates/cn-cli/target/release/organize /usr/local/bin/cn-organizer
    chmod +x /usr/local/bin/cn-organizer
else
    echo "Warning: CLI binary not found at crates/cn-cli/target/release/organize. Did you run 'cargo build --release'?"
fi

# Note: The Tauri GUI is typically installed via the generated .deb or AppImage in target/release/bundle
# This script sets up the CLI tools and raw desktop entries for development/source installations.

echo "Installation complete."
