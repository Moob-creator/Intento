#!/bin/bash

# Test script to verify database separation between debug and release builds

echo "🔍 Checking database file separation..."
echo ""

# Expected paths (updated with separate bundle IDs)
DEV_DIR="$HOME/Library/Application Support/com.intento.app.debug"
PROD_DIR="$HOME/Library/Application Support/com.intento.app"
DEV_DB_PATH="$DEV_DIR/intento.db"
PROD_DB_PATH="$PROD_DIR/intento.db"

echo "📁 Expected database locations:"
echo "  Debug mode:   $DEV_DB_PATH"
echo "  Release mode: $PROD_DB_PATH"
echo ""

# Check debug directory and database
if [ -d "$DEV_DIR" ]; then
    echo "✅ Debug app directory exists"
    if [ -f "$DEV_DB_PATH" ]; then
        echo "✅ Development database found"
        ls -lh "$DEV_DB_PATH"
    else
        echo "⚠️  Development database not found (will be created on first debug run)"
    fi
else
    echo "⚠️  Debug app directory not found (will be created on first debug run)"
fi

echo ""

# Check release directory and database
if [ -d "$PROD_DIR" ]; then
    echo "✅ Release app directory exists"
    if [ -f "$PROD_DB_PATH" ]; then
        echo "✅ Production database found"
        ls -lh "$PROD_DB_PATH"
    else
        echo "⚠️  Production database not found (will be created on first release build run)"
    fi
else
    echo "⚠️  Release app directory not found (will be created on first release build run)"
fi

echo ""
echo "📝 To test:"
echo "  1. Run 'npm run tauri:dev' - should create/use com.intento.app.debug/intento.db"
echo "  2. Run 'npm run tauri:build' then launch the app - should use com.intento.app/intento.db"
echo ""
echo "💡 Benefits:"
echo "  - Complete data isolation between debug and release"
echo "  - Different Bundle IDs mean truly separate apps"
echo "  - Can run both versions simultaneously without conflicts"
echo ""
echo "🔧 Tip: You can use 'sqlite3 <db-file> .tables' to inspect database content"
