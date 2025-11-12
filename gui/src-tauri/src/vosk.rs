pub fn init() -> Result<(), String> {
    info!("Vosk subsystem initialised (stub)");
    Ok(())
}

pub fn recognize(_frame_buffer: &[i16], _partial: bool) -> Option<String> {
    // Stub: в реальной имплементации здесь будет распознавание речи через Vosk
    None
}
