#!/bin/bash

set -e

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$PROJECT_DIR"

echo "==> Building Intento for macOS..."
echo "    Project: $PROJECT_DIR"
echo ""

# Build universal macOS binary
npm run tauri:build -- --target universal-apple-darwin

# Find the generated DMG
DMG_PATH=$(find "$PROJECT_DIR/src-tauri/target/universal-apple-darwin/release/bundle/dmg" -name "*.dmg" 2>/dev/null | head -1)

if [ -z "$DMG_PATH" ]; then
  # Fallback: try native arch
  DMG_PATH=$(find "$PROJECT_DIR/src-tauri/target/release/bundle/dmg" -name "*.dmg" 2>/dev/null | head -1)
fi

echo ""
if [ -n "$DMG_PATH" ]; then
  echo "==> Build succeeded!"
  echo "    DMG: $DMG_PATH"
else
  echo "==> Build finished but DMG not found, check output above."
  exit 1
fi
