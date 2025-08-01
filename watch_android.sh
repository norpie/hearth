#!/bin/bash

# Watch mode Android build script with custom icons and immersive mode
# Automatically rebuilds and reinstalls when source files change

set -e

echo "🔥 Hearth Android Development Watch Mode"
echo "Watching for changes in hearth-ui/, hearth-mobile/, and hearth-core/"
echo "Press Ctrl+C to stop watching"
echo ""

# Function to build and install
build_and_install() {
    echo "📱 Change detected! Rebuilding Android app..."
    ./build_android_with_icons.sh
    echo "✅ Build complete at $(date '+%H:%M:%S')! App should be running on your device."
    echo ""
    echo "👀 Watching for changes..."
}

# Function to cleanup on exit
cleanup() {
    echo ""
    echo "🛑 Stopping watch mode..."
    # Kill any background processes
    jobs -p | xargs -r kill
    exit 0
}

# Set up signal handlers for clean exit
trap cleanup SIGINT SIGTERM

# Check if inotify-tools is available
if ! command -v inotifywait &> /dev/null; then
    echo "❌ inotifywait not found. Please add 'inotify-tools' to your flake.nix dependencies."
    exit 1
fi

# Check if device is connected
if ! command -v adb &> /dev/null || ! adb devices | grep -q "device"; then
    echo "❌ No Android device/emulator connected. Please connect a device first."
    exit 1
fi

# Initial build
echo "🚀 Performing initial build..."
build_and_install

# Watch for changes in source directories
while true; do
    inotifywait -e modify,create,delete,move \
        --include '\.(rs|toml|css|js|html|md)$' \
        -r hearth-ui/ hearth-mobile/ hearth-core/ Cargo.toml Dioxus.toml 2>/dev/null
    
    # Debounce rapid changes
    sleep 2
    
    echo "📁 File change detected, rebuilding..."
    build_and_install
done