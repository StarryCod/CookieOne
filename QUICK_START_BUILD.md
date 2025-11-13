# Quick Start Guide - CookieOne Build Script

## 🚀 Quick Start (3 Steps)

### Step 1: Setup Python Environment

**Linux/macOS:**
```bash
./setup_build_env.sh
```

**Windows:**
```cmd
setup_build_env.bat
```

### Step 2: Activate Virtual Environment

**Linux/macOS:**
```bash
source .venv/bin/activate
```

**Windows:**
```cmd
.venv\Scripts\activate.bat
```

### Step 3: Run Build

```bash
# Standard build
python build_cookieone.py

# Or with options
python build_cookieone.py --release --clean
```

## 📚 Common Use Cases

### Development Build (Fast)
```bash
python build_cookieone.py --skip-tests
```

### Release Build (Optimized)
```bash
python build_cookieone.py --release --clean
```

### Backend Only (No GUI)
```bash
python build_cookieone.py --no-gui
```

### Custom Output Directory
```bash
python build_cookieone.py --output-dir ./my_build
```

## 🎨 Interface Features

The build script provides a beautiful terminal interface with:

- ✅ **Real-time Progress**: Animated progress bars and spinners
- 📊 **System Monitoring**: Live CPU and RAM usage
- 📝 **Live Logs**: Scrolling build output with color coding
- ⏱️ **Timing**: Duration for each stage and total build time
- 🎯 **Stage Status**: Visual indicators for pending/running/success/failed stages

## 🔧 Troubleshooting

### Dependencies Missing
If you see "Missing required dependency", run the setup script:
```bash
./setup_build_env.sh  # Linux/macOS
setup_build_env.bat   # Windows
```

### Build Fails
1. Check the build log in `build_logs/build_YYYYMMDD_HHMMSS.log`
2. Error report is automatically generated in `build_reports/`
3. Look for the failed stage in the terminal output

### Python Version Issues
Ensure Python 3.7+ is installed:
```bash
python3 --version  # Should be 3.7 or higher
```

## 📖 Full Documentation

For detailed documentation, see [BUILD_README.md](BUILD_README.md)

## 🆘 Need Help?

```bash
python build_cookieone.py --help
```

Shows all available options and examples.

---

**That's it! Happy Building! 🎉**
