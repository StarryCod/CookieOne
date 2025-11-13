@echo off
REM Setup script for the CookieOne build environment (Windows)
REM This script creates a Python virtual environment and installs build dependencies

echo 🚀 Setting up CookieOne Build Environment
echo ==========================================
echo.

REM Check if Python is available
python --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Python not found. Please install Python 3.7+ from https://python.org
    exit /b 1
)

echo ✓ Found Python:
python --version

REM Create virtual environment
if not exist ".venv" (
    echo 📦 Creating virtual environment...
    python -m venv .venv
    echo ✓ Virtual environment created
) else (
    echo ✓ Virtual environment already exists
)

REM Activate virtual environment and install dependencies
echo 📥 Activating virtual environment...
call .venv\Scripts\activate.bat

echo 🔄 Upgrading pip...
python -m pip install --upgrade pip >nul 2>&1

echo 📦 Installing build dependencies...
if exist "requirements-build.txt" (
    pip install -r requirements-build.txt
) else (
    pip install rich click colorama psutil
)

echo.
echo ✅ Setup complete!
echo.
echo To use the build script:
echo   1. Activate the virtual environment: .venv\Scripts\activate.bat
echo   2. Run the build script: python build_cookieone.py [OPTIONS]
echo   3. Or run directly: .venv\Scripts\python.exe build_cookieone.py [OPTIONS]
echo.
echo For help: python build_cookieone.py --help
echo.
pause
