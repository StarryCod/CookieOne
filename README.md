# 🍪 Cookie Voice Assistant

A modern, offline-first voice assistant built with Rust, featuring wake word detection, speech-to-text, and JARVIS-style persona.

## ✨ Features

- **✅ Zero C++ Build Dependencies** - Pure Rust implementation using modern libraries
- **🎤 Wake Word Detection** - Custom MicroWakeWord implementation using Vosk
- **🗣️ Speech Recognition** - Dual STT backends:
  - **Vosk** (Default) - Offline, privacy-focused
  - **Gemini Audio** (Optional) - Cloud-based, highly accurate
- **🎭 JARVIS Persona** - Professional, concise responses with randomized phrases
- **🔊 CPAL Audio** - Cross-platform audio input with sample format conversion
- **⚡ Fast & Reliable** - Async/await architecture with Tokio runtime

## 📦 Dependencies

All dependencies are pure Rust with verified versions from crates.io:

### Core Audio & Speech
- `cpal = "0.16"` - Cross-platform audio I/O
- `vosk = "0.3.1"` - Offline speech recognition
- `microwakeword = { path = "../microwakeword" }` - Custom wake word detector

### Networking
- `reqwest = "0.12"` - HTTP client for Gemini API
- `base64 = "0.22"` - Audio encoding for API requests

### Async Runtime
- `tokio = "1.0"` - Async runtime with full features
- `async-trait = "0.1"` - Trait support for async methods

### Utilities
- `serde = "1.0"` - Serialization framework
- `serde_json = "1.0"` - JSON support
- `serde_yaml = "0.9"` - YAML support
- `anyhow = "1.0"` - Error handling
- `fastrand = "2.0"` - Fast random number generation
- `parking_lot = "0.12"` - Faster mutex implementation
- `crossbeam = "0.8"` - Lock-free data structures

## 🚀 Quick Start

### 1. Prerequisites

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Vosk Model** - Download a model:
  - English (small): [vosk-model-small-en-us-0.15](https://alphacephei.com/vosk/models)
  - Russian (small): [vosk-model-small-ru-0.22](https://alphacephei.com/vosk/models)
  - Other languages: [Vosk Models](https://alphacephei.com/vosk/models)

### 2. Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/cookie-voice-assistant
cd cookie-voice-assistant/app

# Download and extract Vosk model
wget https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip
unzip vosk-model-small-en-us-0.15.zip -d assets/stt/

# Build the project
cargo build --release
```

### 3. Configuration

Edit `config.json`:

```json
{
  "wake_word_threshold": 0.45,
  "wake_word_path": "assets/wakeword/cookie.mww",
  "stt_backend": {
    "type": "Vosk",
    "model_path": "assets/stt/vosk-model-small-en-us-0.15"
  },
  "gemini_api_key": null,
  "jarvis_phrases": "assets/phrases/jarvis_style.json",
  "commands_path": "commands/commands.json",
  "listening_device": 0
}
```

### 4. Wake Word Configuration

Edit `assets/wakeword/cookie.mww`:

```json
{
  "model_path": "assets/stt/vosk-model-small-en-us-0.15",
  "keyphrase": "cookie",
  "threshold": 0.45
}
```

**Note:** The wake word model uses the same Vosk model as STT for keyword spotting.

### 5. Run

```bash
cargo run --release
```

Say **"cookie"** to activate, then speak your command!

## 🔧 Advanced Configuration

### Using Gemini Audio STT

1. Get API key from [Google AI Studio](https://makersuite.google.com/app/apikey)
2. Update `config.json`:

```json
{
  "stt_backend": {
    "type": "GeminiAudio"
  },
  "gemini_api_key": "YOUR_API_KEY_HERE"
}
```

### Custom Commands

Edit `commands/commands.json`:

```json
[
  {
    "phrases": ["hello", "hi", "greetings"],
    "action": {
      "type": "shell",
      "command": "echo 'Hello, Sir!'"
    }
  },
  {
    "phrases": ["play music", "music"],
    "action": {
      "type": "shell",
      "command": "vlc ~/Music/playlist.m3u"
    }
  },
  {
    "phrases": ["what time is it", "time"],
    "action": {
      "type": "shell",
      "command": "date '+%H:%M'"
    }
  }
]
```

### JARVIS Persona Customization

Edit `assets/phrases/jarvis_style.json`:

```json
{
  "ack": ["Yes, sir.", "Certainly, sir.", "At your service, sir."],
  "processing": ["Working on it, sir.", "On it, sir."],
  "done": ["Completed, sir.", "All done, sir."],
  "error": ["Sir, an error has occurred.", "I require clarification, sir."],
  "wake": ["I am listening, sir.", "Yes, sir?", "I am here, sir."]
}
```

## 📊 Architecture

### Module Structure

```
app/src/
├── main.rs          # Entry point & main loop
├── config.rs        # Configuration management
├── recorder.rs      # CPAL audio input
├── wakeword.rs      # Wake word detection
├── stt/             # Speech-to-text engines
│   ├── mod.rs       # STT trait & initialization
│   ├── vosk_engine.rs    # Vosk backend
│   └── gemini_audio.rs   # Gemini backend
├── persona.rs       # JARVIS-style phrases
├── audio.rs         # Audio playback
└── commands.rs      # Command matching & execution
```

### Audio Pipeline

```
Microphone → CPAL → Format Conversion → i16 PCM Buffer
                                            ↓
                                    Wake Word Detector
                                            ↓ (activated)
                                    STT Engine (Vosk/Gemini)
                                            ↓
                                    Command Matcher
                                            ↓
                                    Command Executor
```

## 🎯 Key Design Decisions

### 1. **MicroWakeWord Implementation**

Since the `microwakeword` crate doesn't exist on crates.io, we implemented a custom solution using Vosk's grammar-based recognition:

- Uses the same model as STT (no separate wake word model needed)
- Grammar restricts recognition to the wake word only
- Efficient and accurate for keyword spotting
- Simple JSON configuration

### 2. **CPAL for Audio Input**

Replaced PvRecorder with CPAL:
- Pure Rust implementation
- Cross-platform (Windows, macOS, Linux)
- Automatic sample format conversion
- Thread-safe channel-based architecture

### 3. **Vosk 0.3.1 API**

Updated to latest Vosk API:
- `Model::new()` returns `Option<Model>`
- `Recognizer::new_with_grammar()` for wake word
- `CompleteResult` enum with `Single`/`Multiple` variants
- `DecodingState` for waveform acceptance

### 4. **Fastrand for RNG**

Replaced `rand_distr` with `fastrand`:
- Zero dependencies
- Fast non-cryptographic RNG
- Simple API: `fastrand::usize(..phrases.len())`

## 🔍 Troubleshooting

### No Audio Input Detected

```bash
# List audio devices
cargo run --release -- --list-devices
```

Update `listening_device` in `config.json` to the correct device index.

### Wake Word Not Detected

1. **Check microphone** - Ensure it's working and set as default
2. **Lower threshold** - Try `0.3` in `cookie.mww`
3. **Speak clearly** - Say "cookie" clearly and wait ~500ms
4. **Check model** - Ensure Vosk model supports English/your language

### Build Errors

```bash
# Clean build
cargo clean
cargo build --release

# Update dependencies
cargo update
```

### Vosk Model Issues

- Download the correct model for your language
- Extract to `assets/stt/` directory
- Update paths in `config.json` and `cookie.mww`
- Ensure paths are relative to the app directory

## 📝 Version Compatibility

All dependencies have been tested and verified:

| Dependency | Version | Status |
|------------|---------|--------|
| cpal | 0.16 | ✅ Latest stable |
| vosk | 0.3.1 | ✅ Latest on crates.io |
| reqwest | 0.12 | ✅ Latest stable |
| tokio | 1.0 | ✅ LTS release |
| serde | 1.0 | ✅ Stable |
| parking_lot | 0.12 | ✅ Stable |
| crossbeam | 0.8 | ✅ Stable |

## 🚧 Known Limitations

1. **Audio Playback** - Currently placeholder (logs only)
   - Plan to integrate `rodio` or `kira` for TTS
2. **macOS Tray** - Tray icon support limited on macOS
3. **Windows Only Shell Commands** - Uses `sh -c`, adapt for Windows PowerShell if needed

## 🗺️ Roadmap

- [ ] Implement audio playback with Rodio
- [ ] Add TTS support (Silero-rs)
- [ ] GUI application with Tauri
- [ ] Settings UI for device selection
- [ ] Multiple wake word support
- [ ] Command history & analytics
- [ ] Plugin system for custom actions

## 📄 License

GPL-3.0-only - See LICENSE.txt

## 🙏 Acknowledgments

- **Vosk** - Offline speech recognition by Alpha Cephei
- **CPAL** - Cross-platform audio library
- **Google Gemini** - Cloud-based speech recognition API
- **Jarvis (MCU)** - Inspiration for the persona style

## 📞 Support

- Report issues on GitHub
- For questions, open a discussion
- Contributions welcome via pull requests

---

**Built with ❤️ in Rust** | No C++ Required | Privacy-First | Offline-Capable
