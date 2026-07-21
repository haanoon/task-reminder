#!/bin/bash
# Installation script for Wallpaper Tasks

set -e

echo "Installing Wallpaper Tasks..."

# Build release binary
cargo build --release

# Install binary to ~/.local/bin
mkdir -p ~/.local/bin
cp target/release/wallpaper-tasks ~/.local/bin/
chmod +x ~/.local/bin/wallpaper-tasks

echo "✓ Binary installed to ~/.local/bin/wallpaper-tasks"

# Create XDG autostart entry
mkdir -p ~/.config/autostart
cat > ~/.config/autostart/wallpaper-tasks.desktop << 'EOF'
[Desktop Entry]
Type=Application
Name=Wallpaper Tasks
Comment=Task manager on desktop wallpaper
Exec=$HOME/.local/bin/wallpaper-tasks
Icon=task-due
Terminal=false
Categories=Utility;GTK;
X-GNOME-Autostart-enabled=true
StartupNotify=false
EOF

echo "✓ XDG autostart entry created"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "⚠ WARNING: ~/.local/bin is not in your PATH"
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo ""
echo "✓ Installation complete!"
echo ""
echo "The app will auto-start on next login."
echo "To start it now, run: wallpaper-tasks"
