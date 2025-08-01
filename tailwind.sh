#!/bin/bash

# Tailwind CSS build/watch script using entr
# Monitors source files and regenerates CSS for all subcrates

if [[ "$1" == "--build" ]]; then
    echo "🎨 Building Tailwind CSS once..."
else
    echo "🎨 Starting Tailwind CSS watcher..."
fi

# Function to build Tailwind CSS for shared ui
build_tailwind() {
    echo "🔄 Rebuilding Tailwind CSS..."
    
    # Ensure output directory and file exist
    mkdir -p hearth-ui/assets
    touch hearth-ui/assets/tailwind.css
    
    # Generate CSS for shared ui only (unified UI architecture)
    npx @tailwindcss/cli -i ./hearth-ui/app.css -o hearth-ui/assets/tailwind.css
    
    echo "✅ Tailwind CSS updated for hearth-ui/ (unified UI)"
}

# Handle build-only mode (called by entr or user)
if [[ "$1" == "--build-only" ]] || [[ "$1" == "--build" ]]; then
    build_tailwind
    exit 0
fi

# Initial build for watch mode
build_tailwind

# Watch for changes in source files and config
echo "👀 Watching for changes in:"
echo "  - ./hearth-web/src/**/*.rs"
echo "  - ./hearth-desktop/src/**/*.rs" 
echo "  - ./hearth-mobile/src/**/*.rs"
echo "  - ./hearth-ui/src/**/*.rs"
echo "  - ./hearth-ui/app.css"
echo "  - ./hearth-ui/assets/*.css"
echo "  - ./tailwind.config.js"
echo ""

find ./hearth-web/src ./hearth-desktop/src ./hearth-mobile/src ./hearth-ui/src ./hearth-ui/app.css ./hearth-ui/assets ./tailwind.config.js -name "*.rs" -o -name "*.html" -o -name "*.css" -o -name "*.js" | entr -s 'bash -c "echo \"📁 File changed, rebuilding...\" && ./tailwind.sh --build-only"'