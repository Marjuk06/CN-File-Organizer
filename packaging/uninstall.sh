#!/bin/bash
set -e

# CN File Organizer Uninstallation Script

if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (use sudo)"
  exit 1
fi

echo "Uninstalling CN File Organizer..."

# Remove desktop entry
if [ -f "/usr/share/applications/cn-organizer.desktop" ]; then
    rm /usr/share/applications/cn-organizer.desktop
fi

# Remove CLI binary
if [ -f "/usr/local/bin/cn-organizer" ]; then
    rm /usr/local/bin/cn-organizer
fi

echo "Uninstallation complete."
