#!/usr/bin/env bash
set -e

# --- Get version from Cargo.toml ---
VERSION=$(grep -m 1 '^[[:space:]]*version[[:space:]]*=' Cargo.toml | sed -E 's/^[[:space:]]*version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/')
echo "Building obs-reminder v$VERSION"

# --- Build browser frontend ---
cd browser
pnpm install
pnpm run build
cd ..

# --- Build Rust application ---
cargo build --release

# --- Create output folder ---
OUTPUT_DIR="obs-reminder-linux-x64-$VERSION-stable"
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# --- Copy executable ---
cp target/release/obs-reminder "$OUTPUT_DIR/"

# --- Copy browser build ---
cp -r browser/dist "$OUTPUT_DIR/browser"
zip -r "$OUTPUT_DIR.zip" "$OUTPUT_DIR"
rm -rf "$OUTPUT_DIR"

echo
echo "Build completed: $OUTPUT_DIR.zip"
