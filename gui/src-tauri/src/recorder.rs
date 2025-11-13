use std::sync::atomic::{AtomicU32, Ordering};

pub static FRAME_LENGTH: AtomicU32 = AtomicU32::new(512);

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u32)]
pub enum RecorderType {
    Cpal = 0,
    PvRecorder = 1,
    PortAudio = 2,
}

impl From<u32> for RecorderType {
    fn from(v: u32) -> Self {
        match v {
            0 => RecorderType::Cpal,
            1 => RecorderType::PvRecorder,
            2 => RecorderType::PortAudio,
            _ => RecorderType::PvRecorder,
        }
    }
}

pub static RECORDER_TYPE: AtomicU32 = AtomicU32::new(RecorderType::PvRecorder as u32);

pub fn init() -> Result<(), String> {
    RECORDER_TYPE.store(RecorderType::PvRecorder as u32, Ordering::SeqCst);
    Ok(())
}

pub fn start_recording() {
    info!("Recorder: start_recording");
}

pub fn stop_recording() {
    info!("Recorder: stop_recording");
}

pub fn read_microphone(_frame_buffer: &mut [i16]) {
    // Placeholder: в реальной имплементации здесь чтение с микрофона
}
