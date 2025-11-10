# 🎤 MicroWakeWord

A lightweight wake word detection library for Rust, built on top of Vosk speech recognition.

## Features

- ✅ **Pure Rust** - No C++ dependencies beyond Vosk
- ⚡ **Fast** - Grammar-based recognition for efficient keyword spotting
- 🔧 **Configurable** - JSON-based configuration files
- 🎯 **Simple API** - Just two main methods: `from_config_file()` and `process()`
- 🌍 **Multi-language** - Supports any language that Vosk supports

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
microwakeword = { path = "../microwakeword" }  # or version = "0.2" when published
```

## Usage

### 1. Create a configuration file (`wake.mww`):

```json
{
  "model_path": "path/to/vosk-model",
  "keyphrase": "cookie",
  "threshold": 0.45
}
```

### 2. Initialize the detector:

```rust
use microwakeword::WakeWordDetector;

let mut detector = WakeWordDetector::from_config_file(
    "wake.mww",
    16000.0  // sample rate
)?;
```

### 3. Process audio:

```rust
let audio_buffer: Vec<i16> = get_audio_from_microphone();

if detector.process(&audio_buffer) {
    println!("Wake word detected!");
}
```

## How It Works

MicroWakeWord uses Vosk's grammar-based recognition to restrict the search space to only your wake word:

1. **Load Vosk Model** - Uses the same model as your main STT system
2. **Apply Grammar** - Restricts recognition to the specified keyphrase only
3. **Fast Matching** - Only matches against the single keyword, not full vocabulary
4. **Low Latency** - Processes audio in real-time with minimal overhead

## Configuration

### Config File Format

```json
{
  "model_path": "assets/stt/vosk-model-small-en-us-0.15",
  "keyphrase": "cookie",
  "threshold": 0.45
}
```

- **model_path** - Path to Vosk model directory (relative or absolute)
- **keyphrase** - The wake word to detect (lowercase recommended)
- **threshold** - Confidence threshold (0.0-1.0, default: 0.45)

### Choosing a Threshold

- **0.3-0.4** - More sensitive, may have false positives
- **0.45-0.5** - Balanced (recommended)
- **0.6-0.7** - More strict, fewer false positives but may miss detections

## API Reference

### `WakeWordDetector`

```rust
pub struct WakeWordDetector {
    // Internal fields hidden
}
```

#### Methods

##### `from_config_file(path, sample_rate)`

Load detector from JSON configuration file.

```rust
pub fn from_config_file<P: AsRef<Path>>(
    path: P, 
    sample_rate: f32
) -> Result<Self>
```

**Parameters:**
- `path` - Path to `.mww` configuration file
- `sample_rate` - Audio sample rate (typically 16000.0 Hz)

**Returns:** `Result<WakeWordDetector>`

**Errors:**
- File not found
- Invalid JSON format
- Vosk model load failure
- Invalid model path

##### `process(pcm)`

Process audio buffer and detect wake word.

```rust
pub fn process(&mut self, pcm: &[i16]) -> bool
```

**Parameters:**
- `pcm` - Audio samples as 16-bit signed integers

**Returns:** `true` if wake word detected, `false` otherwise

##### `phrase()`

Get the configured wake word phrase.

```rust
pub fn phrase(&self) -> &str
```

##### `threshold()`

Get the configured confidence threshold.

```rust
pub fn threshold(&self) -> f32
```

## Example: Complete Integration

```rust
use microwakeword::WakeWordDetector;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> anyhow::Result<()> {
    // Initialize detector
    let mut detector = WakeWordDetector::from_config_file(
        "assets/wakeword/cookie.mww",
        16000.0
    )?;

    println!("Listening for wake word: '{}'", detector.phrase());

    // Set up audio input (using CPAL)
    let host = cpal::default_host();
    let device = host.default_input_device()
        .expect("No input device");
    
    let config = device.default_input_config()?;
    
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[i16], _| {
            if detector.process(data) {
                println!("Wake word detected!");
            }
        },
        |err| eprintln!("Error: {}", err),
        None
    )?;

    stream.play()?;

    // Keep running
    std::thread::park();
    
    Ok(())
}
```

## Performance

- **CPU Usage**: ~5-10% on modern CPUs (single core)
- **Memory**: ~100-200 MB (depends on Vosk model size)
- **Latency**: <100ms typical response time
- **Buffer Size**: Processes 512-1024 sample frames efficiently

## Supported Vosk Models

Works with any Vosk model, but smaller models are recommended for wake word detection:

- **English**: `vosk-model-small-en-us-0.15` (~40 MB)
- **Russian**: `vosk-model-small-ru-0.22` (~45 MB)  
- **Other**: See [Vosk Models](https://alphacephei.com/vosk/models)

## Troubleshooting

### Wake Word Not Detected

1. **Check pronunciation** - Say the word clearly
2. **Lower threshold** - Try 0.3-0.4
3. **Check microphone** - Ensure proper audio input
4. **Verify model** - Model must support your language

### High CPU Usage

1. **Use smaller model** - Switch to "small" variant
2. **Increase buffer size** - Process larger chunks
3. **Reduce sample rate** - If possible (may affect accuracy)

### False Positives

1. **Increase threshold** - Try 0.5-0.6
2. **Choose distinct word** - Avoid common words
3. **Better model** - Use larger model if CPU allows

## Why Not Use Specialized Wake Word Engines?

MicroWakeWord takes a different approach:

| Feature | MicroWakeWord | Specialized (Porcupine, etc) |
|---------|---------------|------------------------------|
| Model Size | Shares with STT | Separate model needed |
| Dependencies | Pure Rust (via Vosk) | Often requires C++ |
| Flexibility | Any word/phrase | Pre-trained keywords only |
| Language Support | 20+ languages | Limited languages |
| Accuracy | Good (grammar-based) | Excellent (specialized) |
| CPU Usage | Low-Medium | Very Low |

## Dependencies

```toml
[dependencies]
vosk = "0.3.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
log = "0.4"
```

## License

GPL-3.0-only

## Contributing

Contributions welcome! Please:

1. Test with multiple Vosk models
2. Benchmark performance changes
3. Update documentation
4. Add examples for new features

## Acknowledgments

Built on top of [Vosk](https://alphacephei.com/vosk/) by Alpha Cephei.
