#!/bin/bash

# Function to generate icon with minimal padding
generate_icon() {
    local size=$1
    local output_file=$2
    
    echo "Generating ${size}x${size} icon with maximum fill..."
    
    nix-shell -p inkscape --run "inkscape \
        -w ${size} \
        -h ${size} \
        --export-area-drawing \
        --export-type=png \
        --export-filename=${output_file} \
        hearth-ui/assets/icons/logo.svg"
}

# Generate all required icon sizes
echo "Generating Android app icons with minimal padding..."

generate_icon 32 "hearth-ui/assets/icons/logo-32.png"
generate_icon 48 "hearth-ui/assets/icons/logo-48.png"
generate_icon 64 "hearth-ui/assets/icons/logo-64.png"
generate_icon 72 "hearth-ui/assets/icons/logo-72.png"
generate_icon 96 "hearth-ui/assets/icons/logo-96.png"
generate_icon 128 "hearth-ui/assets/icons/logo-128.png"
generate_icon 144 "hearth-ui/assets/icons/logo-144.png"
generate_icon 192 "hearth-ui/assets/icons/logo-192.png"
generate_icon 256 "hearth-ui/assets/icons/logo-256.png"
generate_icon 512 "hearth-ui/assets/icons/logo-512.png"
generate_icon 1024 "hearth-ui/assets/icons/logo-1024.png"

echo "All icons generated successfully!"

# Generate ICO favicon for web
echo "Generating ICO favicon for web browsers..."

# First generate intermediate PNG sizes for ICO
nix-shell -p inkscape --run "inkscape \
    -w 16 \
    -h 16 \
    --export-area-drawing \
    --export-type=png \
    --export-filename=hearth-ui/assets/icons/favicon-16.png \
    hearth-ui/assets/icons/logo.svg"

nix-shell -p inkscape --run "inkscape \
    -w 32 \
    -h 32 \
    --export-area-drawing \
    --export-type=png \
    --export-filename=hearth-ui/assets/icons/favicon-32.png \
    hearth-ui/assets/icons/logo.svg"

nix-shell -p inkscape --run "inkscape \
    -w 48 \
    -h 48 \
    --export-area-drawing \
    --export-type=png \
    --export-filename=hearth-ui/assets/icons/favicon-48.png \
    hearth-ui/assets/icons/logo.svg"

# Convert PNGs to multi-size ICO file
nix-shell -p imagemagick --run "magick convert \
    hearth-ui/assets/icons/favicon-16.png \
    hearth-ui/assets/icons/favicon-32.png \
    hearth-ui/assets/icons/favicon-48.png \
    hearth-ui/assets/icons/favicon.ico"

# Copy ICO to web assets
cp hearth-ui/assets/icons/favicon.ico hearth-web/assets/favicon.ico

# Clean up intermediate files
rm hearth-ui/assets/icons/favicon-16.png hearth-ui/assets/icons/favicon-32.png hearth-ui/assets/icons/favicon-48.png

echo "ICO favicon generated successfully!"