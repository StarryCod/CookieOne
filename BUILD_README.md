# CookieOne Build Script

## 🚀 Overview

A comprehensive cross-platform Python build script with a beautiful, animated terminal interface for building the CookieOne Voice Assistant project.

## ✨ Features

### Beautiful Interface
- **Dark Theme**: Modern dark gray terminal interface with vibrant colors
- **Real-time Progress**: Animated progress bars with percentage completion
- **Live Status Updates**: See exactly what's being built in real-time
- **Scrollable Logs**: Last 20-100 lines visible with automatic scrolling
- **System Monitoring**: CPU and memory usage display
- **Stage Indicators**: Visual status for each build stage (✅ ❌ 🔄 ⏳)

### Animations
- **Spinner Loader**: Multiple spinner styles for active operations
- **Progress Bar Pulse**: Pulsing effect during active builds
- **Smooth Transitions**: Fade effects between stages
- **Stage Highlighting**: Active stage pulses with cyan highlight
- **Colored Logs**: Syntax highlighting for errors, warnings, and success messages

### Build Capabilities
- **Multi-Stage Pipeline**: Orchestrates all build phases
  1. Environment Check (Rust, Cargo, Node.js, npm)
  2. Repository Preparation (git branch, submodules)
  3. Clean Build Artifacts (optional)
  4. Build Rust Core (/app)
  5. Run Tests (optional)
  6. Build Frontend (Svelte/Vite)
  7. Build Tauri Application
  8. Package Distribution
  9. Finalization
- **Error Handling**: Graceful error handling with detailed reporting
- **Logging**: Comprehensive build logs saved to `build_logs/` directory
- **Error Reports**: Automatic error report generation with system info

### Interactivity (when running in TTY)
- **Start**: Begin the build process
- **Pause**: Pause between stages
- **Resume**: Continue a paused build
- **Cancel**: Gracefully stop the build
- **Export Report**: Generate error report on demand

## 📋 Requirements

### System Dependencies
- **Python 3.7+**
- **Rust & Cargo** (for Rust builds)
- **Node.js & npm** (for GUI builds)
- **Git** (for repository operations)

### Python Dependencies
```bash
pip install -r requirements-build.txt
```

Or install individually:
```bash
pip install rich click colorama psutil
```

## 🎮 Usage

### Basic Usage
```bash
# Standard debug build
python build_cookieone.py

# Or make it executable and run directly
chmod +x build_cookieone.py
./build_cookieone.py
```

### Command-Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--branch TEXT` | Git branch to build | `final-release-wakeword-windows-build-cargo-check` |
| `--release` | Build in release mode (optimized) | `False` |
| `--clean` | Clean build artifacts before building | `False` |
| `--no-gui` | Skip GUI build (backend only) | `False` |
| `--skip-tests` | Skip running tests | `False` |
| `--output-dir PATH` | Output directory for build artifacts | `dist/` |
| `--verbose` | Verbose output (more log lines) | `False` |
| `--help` | Show help message | - |
| `--version` | Show version | - |

### Examples

#### Release Build with Cleanup
```bash
python build_cookieone.py --release --clean
```

#### Backend Only (No GUI)
```bash
python build_cookieone.py --no-gui
```

#### Quick Build (Skip Tests)
```bash
python build_cookieone.py --skip-tests
```

#### Custom Output Directory
```bash
python build_cookieone.py --release --output-dir /path/to/output
```

#### Verbose Mode
```bash
python build_cookieone.py --verbose
```

#### Specific Branch
```bash
python build_cookieone.py --branch my-feature-branch
```

#### Combined Options
```bash
python build_cookieone.py --release --clean --skip-tests --output-dir ./release_build
```

## 📊 Output

### Build Artifacts
By default, build artifacts are placed in:
- `dist/` - Final packaged applications
- `app/target/` - Rust build artifacts
- `gui/dist/` - Frontend build
- `gui/src-tauri/target/` - Tauri bundles

### Logs
- `build_logs/build_YYYYMMDD_HHMMSS.log` - Detailed build log
- `build_reports/error_report_YYYYMMDD_HHMMSS.zip` - Error report (on failure)

### Build Manifest
After successful build, a `build_manifest.json` is created in the output directory containing:
- Build configuration
- Platform information
- Stage timings
- Build timestamp

## 🎨 Interface Guide

### Header
```
╔═══════════════════════════════════════════════════════════╗
║  🚀 COOKIEONE VOICE ASSISTANT 🚀                         ║
╚═══════════════════════════════════════════════════════════╝
    Version 1.0.0 • Build System 🔄
```

### Progress Panel
Shows overall build progress with:
- Percentage completion
- Completed stages count
- Animated progress bar
- Elapsed time

### Stages Table
Lists all build stages with:
- Stage number
- Stage name
- Status icon (⏳ 🔄 ✅ ❌)
- Duration

### Log Panel
Displays last 20 lines of build output with color coding:
- 🟢 Green: Success messages
- 🔴 Red: Errors
- 🟡 Yellow: Warnings
- ⚪ White: Info messages

### System Resources Panel
Real-time monitoring:
- CPU usage with visual bar
- RAM usage with visual bar

## 🐛 Troubleshooting

### Missing Dependencies
If you see "Missing required tools", install them:
- **Rust**: https://rustup.rs/
- **Node.js**: https://nodejs.org/

### Build Failures
1. Check the build log in `build_logs/`
2. Generate error report (automatic on failure)
3. Review the failed stage details

### Permission Errors
On Unix systems, ensure the script is executable:
```bash
chmod +x build_cookieone.py
```

### Python Dependencies
If dependencies fail to auto-install:
```bash
pip install --upgrade pip
pip install -r requirements-build.txt
```

## 🔧 Advanced

### Environment Variables
The script respects existing environment variables and can be customized:
```bash
export CARGO_BUILD_JOBS=4
export NODE_OPTIONS="--max-old-space-size=4096"
python build_cookieone.py
```

### Platform-Specific Notes

#### Windows
- Ensure Python is in PATH
- Run from PowerShell or Command Prompt
- May require Visual Studio Build Tools for Rust

#### Linux
- Ensure build dependencies are installed
- May need `libgtk-3-dev`, `libwebkit2gtk-4.0-dev` for Tauri

#### macOS
- Xcode Command Line Tools required
- May need to allow apps from unidentified developers

## 📝 License

GPL-3.0 - Same as the CookieOne project

## 🤝 Contributing

Improvements to the build script are welcome! Please ensure:
- Cross-platform compatibility (Windows, Linux, macOS)
- Clear error messages
- Maintain the visual aesthetics
- Add tests for new features

## 🎯 Future Enhancements

Potential improvements:
- [ ] Parallel stage execution where possible
- [ ] Build caching support
- [ ] Docker container builds
- [ ] CI/CD integration helpers
- [ ] Build profile presets
- [ ] Incremental build support
- [ ] Web-based build monitoring

## 📞 Support

For issues related to:
- **Build Script**: Open an issue on the repository
- **Project Build**: Check the main project documentation
- **Dependencies**: Refer to official documentation for Rust, Node.js, Tauri

---

**Happy Building! 🚀**
