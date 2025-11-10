use crate::{config, APP_DIR};
use anyhow::Result;
use microwakeword::WakeWordDetector;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;

static DETECTOR: OnceCell<Mutex<WakeWordDetector>> = OnceCell::new();

pub fn init() -> Result<()> {
    info!("Initializing wake word detector...");

    let cfg = config::config();
    let path = APP_DIR.join(&cfg.wake_word_path);

    let detector = WakeWordDetector::from_config_file(path, 16000.0)?;
    info!(
        "Wake word detector initialized: phrase='{}', threshold={}",
        detector.phrase(),
        detector.threshold()
    );

    DETECTOR
        .set(Mutex::new(detector))
        .map_err(|_| anyhow::anyhow!("Detector already initialized"))?;

    Ok(())
}

pub fn detect(pcm: &[i16]) -> bool {
    if let Some(detector) = DETECTOR.get() {
        detector.lock().process(pcm)
    } else {
        false
    }
}
