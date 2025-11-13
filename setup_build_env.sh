#!/bin/bash
# Setup script for the CookieOne build environment
# This script creates a Python virtual environment and installs build dependencies

set -e

echo "🚀 Setting up CookieOne Build Environment"
echo "=========================================="
echo ""

# Check Python version
PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
echo "✓ Found Python: $PYTHON_VERSION"

# Check if venv module is available
if ! python3 -m venv --help >/dev/null 2>&1; then
    echo "❌ Python venv module not available"
    echo "Please install: sudo apt-get install python3-venv"
    exit 1
fi

# Create virtual environment
if [ ! -d ".venv" ]; then
    echo "📦 Creating virtual environment..."
    python3 -m venv .venv
    echo "✓ Virtual environment created"
else
    echo "✓ Virtual environment already exists"
fi

# Activate virtual environment
echo "📥 Activating virtual environment..."
source .venv/bin/activate

# Upgrade pip
echo "🔄 Upgrading pip..."
pip install --upgrade pip >/dev/null 2>&1

# Install dependencies
echo "📦 Installing build dependencies..."
if [ -f "requirements-build.txt" ]; then
    pip install -r requirements-build.txt
else
    pip install rich click colorama psutil
fi

echo ""
echo "✅ Setup complete!"
echo ""
echo "To use the build script:"
echo "  1. Activate the virtual environment: source .venv/bin/activate"
echo "  2. Run the build script: python build_cookieone.py [OPTIONS]"
echo "  3. Or run directly: .venv/bin/python build_cookieone.py [OPTIONS]"
echo ""
echo "For help: python build_cookieone.py --help"
