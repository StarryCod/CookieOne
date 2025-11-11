// mod pvrecorder;  // Disabled: pv_recorder crate is yanked from crates.io
mod cpal;
// mod portaudio;

use once_cell::sync::OnceCell;

use crate::{DB, config, config::structs::RecorderType};

static RECORDER_TYPE: OnceCell<RecorderType> = OnceCell::new();
static FRAME_LENGTH: OnceCell<u32> = OnceCell::new();

pub fn init() -> Result<(), ()> {
    // set default recorder type
    RECORDER_TYPE.set(config::DEFAULT_RECORDER_TYPE).unwrap();
    FRAME_LENGTH.set(512u32).unwrap();

    // load given recorder
    match RECORDER_TYPE.get().unwrap() {
        RecorderType::Cpal => {
            // Init CPAL
            info!("Initializing CPAL recording backend");
            match cpal::init_microphone(get_selected_microphone_index(), FRAME_LENGTH.get().unwrap().to_owned()) {
                false => {
                    error!("CPAL recorder initialization failed.");
                    return Err(())
                },
                _ => {
                    info!("CPAL recorder initialization success.");
                }
            }
        }
    }

    Ok(())
}

pub fn read_microphone(frame_buffer: &mut [i16]) {
    match RECORDER_TYPE.get().unwrap() {
        RecorderType::Cpal => {
            cpal::read_microphone(frame_buffer);
        }
    }
}

pub fn start_recording() -> Result<(), ()> {
    match RECORDER_TYPE.get().unwrap() {
        RecorderType::Cpal => {
            cpal::start_recording(get_selected_microphone_index(), FRAME_LENGTH.get().unwrap().to_owned());
            Ok(())
        }
    }
}

pub fn stop_recording() -> Result<(), ()> {
    match RECORDER_TYPE.get().unwrap() {
        RecorderType::Cpal => {
            cpal::stop_recording();
            Ok(())
        }
    }
}

pub fn get_selected_microphone_index() -> i32 {
    DB.get().unwrap().lock().unwrap().microphone
}