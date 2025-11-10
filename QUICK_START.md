# 🚀 Quick Start Guide - Cookie Voice Assistant

## Prerequisites

- **Rust 1.70+** ([Install](https://rustup.rs/))
- **Operating System:** Windows, macOS, or Linux
- **Microphone:** Any USB or built-in microphone

## 5-Minute Setup

### Step 1: Download Vosk Model (2 minutes)

```bash
cd app/assets/stt/

# English (Small - Recommended for testing)
wget https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip
unzip vosk-model-small-en-us-0.15.zip
rm vosk-model-small-en-us-0.15.zip

# OR Russian (if preferred)
# wget https://alphacephei.com/vosk/models/vosk-model-small-ru-0.22.zip
# unzip vosk-model-small-ru-0.22.zip
# rm vosk-model-small-ru-0.22.zip
```

**Result:** You should have `app/assets/stt/vosk-model-small-en-us-0.15/` directory

### Step 2: Build (2 minutes)

```bash
cd app
cargo build --release
```

**Note:** First build downloads dependencies (~200MB), takes 2-3 minutes

### Step 3: Run (1 minute)

```bash
cargo run --release
```

**Expected output:**
```
INFO - Starting Cookie Voice Assistant v0.0.3
INFO - Initializing CPAL audio recorder...
INFO - Using audio device: Default Input Device
INFO - CPAL audio recorder initialized successfully
INFO - Initializing wake word detector...
INFO - Wake word detector initialized: phrase='cookie', threshold=0.45
INFO - Vosk STT engine initialized
INFO - Starting main loop...
```

### Step 4: Test Wake Word

1. **Wait** for initialization to complete
2. **Say clearly:** "cookie"
3. **Expected:** Console prints "Wake word detected!"
4. **Then say:** "hello" or "hi"
5. **Expected:** Command executes (echoes "Hello, Sir!")

## Troubleshooting

### Problem: No audio input detected

**Solution 1:** Check microphone permissions
```bash
# Windows: Settings → Privacy → Microphone
# macOS: System Preferences → Security & Privacy → Microphone
# Linux: Check alsa/pulseaudio settings
```

**Solution 2:** List available devices
```bash
# TODO: Add device listing command
# For now, check system audio settings
```

### Problem: Wake word not detected

**Try lowering threshold:**

Edit `app/assets/wakeword/cookie.mww`:
```json
{
  "model_path": "assets/stt/vosk-model-small-en-us-0.15",
  "keyphrase": "cookie",
  "threshold": 0.3  // Lower = more sensitive
}
```

**Tips:**
- Speak clearly and at normal volume
- Say "cookie" as a single word
- Wait ~500ms after saying it
- Try in a quiet environment

### Problem: Build fails

**Check Rust version:**
```bash
rustc --version  # Should be 1.70+
rustup update
```

**Clean and rebuild:**
```bash
cd app
cargo clean
cargo build --release
```

### Problem: Model not found

**Verify directory structure:**
```bash
cd app
ls -la assets/stt/vosk-model-small-en-us-0.15/
```

**Should contain:**
- `am/` directory
- `conf/` directory
- `graph/` directory
- `README`

## Using Gemini Audio API (Optional)

### Step 1: Get API Key

1. Visit [Google AI Studio](https://makersuite.google.com/app/apikey)
2. Click "Create API Key"
3. Copy the key

### Step 2: Update Config

Edit `app/config.json`:
```json
{
  "wake_word_threshold": 0.45,
  "wake_word_path": "assets/wakeword/cookie.mww",
  "stt_backend": {
    "type": "GeminiAudio"  // Changed from "Vosk"
  },
  "gemini_api_key": "YOUR_API_KEY_HERE",  // Paste your key
  "jarvis_phrases": "assets/phrases/jarvis_style.json",
  "commands_path": "commands/commands.json",
  "listening_device": 0
}
```

### Step 3: Restart

```bash
cargo run --release
```

**Benefits:**
- ✅ More accurate transcription
- ✅ Better noise handling
- ⚠️ Requires internet connection
- ⚠️ Sends audio to Google servers

## Adding Custom Commands

Edit `app/commands/commands.json`:

```json
[
  {
    "phrases": ["open browser", "browser"],
    "action": {
      "type": "shell",
      "command": "firefox"
    }
  },
  {
    "phrases": ["what time", "time"],
    "action": {
      "type": "shell",
      "command": "date"
    }
  },
  {
    "phrases": ["volume up", "louder"],
    "action": {
      "type": "shell",
      "command": "amixer set Master 10%+"
    }
  }
]
```

**Restart** after editing to load new commands.

## Customizing Persona

Edit `app/assets/phrases/jarvis_style.json`:

```json
{
  "ack": [
    "Yes, sir.",
    "Certainly, sir.",
    "Of course, sir.",
    "Right away, sir."
  ],
  "processing": [
    "Working on it, sir.",
    "On it, sir.",
    "Processing, sir."
  ],
  "done": [
    "Completed, sir.",
    "All done, sir.",
    "Task finished, sir.",
    "Ready, sir."
  ],
  "error": [
    "Sir, an error has occurred.",
    "I require clarification, sir.",
    "This action is unavailable, sir."
  ],
  "wake": [
    "I am listening, sir.",
    "Yes, sir?",
    "I am here, sir.",
    "At your service, sir."
  ]
}
```

**No restart required** - phrases are reloaded on each use.

## Performance Tips

### Reduce CPU Usage

Use a smaller Vosk model:
```bash
# Tiny model (fastest, less accurate)
wget https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip
```

### Increase Accuracy

Use a larger model:
```bash
# Large model (slower, more accurate)
wget https://alphacephei.com/vosk/models/vosk-model-en-us-0.22.zip
```

Update paths in `config.json` and `cookie.mww`

## Next Steps

- 📖 Read [full README](README.md) for advanced features
- 📝 Read [implementation summary](IMPLEMENTATION_SUMMARY.md) for technical details
- 🔧 Customize commands for your workflow
- 🎤 Adjust wake word threshold for your environment
- 🚀 Build something awesome!

## Common Commands (Examples)

Once running, try these voice commands:

1. **"cookie"** → Wake the assistant
2. **"hello"** → Test basic command
3. **"hi"** → Alternative greeting
4. Add your own in `commands/commands.json`!

## Getting Help

- 📖 [Full Documentation](README.md)
- 🐛 [Report Issues](https://github.com/yourusername/cookie/issues)
- 💬 [Discussions](https://github.com/yourusername/cookie/discussions)
- 📧 Email support (if available)

## Success! 🎉

If you see:
```
INFO - Wake word detector initialized: phrase='cookie'
INFO - Vosk STT engine initialized
INFO - Starting main loop...
```

**You're ready to go!** Say "cookie" and start issuing commands.

---

**Need more details?** Check [README.md](README.md) for comprehensive documentation.

**Developers?** See [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) for technical deep-dive.
