use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, StreamConfig};
use crossbeam::channel::{bounded, Receiver, Sender};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::sync::Arc;

static AUDIO_STREAM: OnceCell<Arc<Mutex<Option<Stream>>>> = OnceCell::new();
static AUDIO_RECEIVER: OnceCell<Receiver<Vec<i16>>> = OnceCell::new();

pub fn init() -> Result<()> {
    info!("Initializing CPAL audio recorder...");

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .context("No default audio input device available")?;

    info!("Using audio device: {}", device.name()?);

    let config = device
        .default_input_config()
        .context("Failed to get default input config")?;

    let (sender, receiver) = bounded::<Vec<i16>>(100);

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => build_stream_i8(&device, &config.into(), sender)?,
        cpal::SampleFormat::I16 => build_stream_i16(&device, &config.into(), sender)?,
        cpal::SampleFormat::I32 => build_stream_i32(&device, &config.into(), sender)?,
        cpal::SampleFormat::I64 => build_stream_i64(&device, &config.into(), sender)?,
        cpal::SampleFormat::U8 => build_stream_u8(&device, &config.into(), sender)?,
        cpal::SampleFormat::U16 => build_stream_u16(&device, &config.into(), sender)?,
        cpal::SampleFormat::U32 => build_stream_u32(&device, &config.into(), sender)?,
        cpal::SampleFormat::U64 => build_stream_u64(&device, &config.into(), sender)?,
        cpal::SampleFormat::F32 => build_stream_f32(&device, &config.into(), sender)?,
        cpal::SampleFormat::F64 => build_stream_f64(&device, &config.into(), sender)?,
        _ => anyhow::bail!("Unsupported sample format"),
    };

    AUDIO_STREAM
        .set(Arc::new(Mutex::new(Some(stream))))
        .map_err(|_| anyhow::anyhow!("Audio stream already initialized"))?;

    AUDIO_RECEIVER
        .set(receiver)
        .map_err(|_| anyhow::anyhow!("Audio receiver already initialized"))?;

    info!("CPAL audio recorder initialized successfully");
    Ok(())
}

pub fn start() -> Result<()> {
    if let Some(stream_mutex) = AUDIO_STREAM.get() {
        if let Some(stream) = stream_mutex.lock().as_ref() {
            stream.play()?;
            info!("Audio recording started");
            return Ok(());
        }
    }
    anyhow::bail!("Audio stream not initialized")
}

pub fn read(buffer: &mut [i16]) -> Result<()> {
    if let Some(receiver) = AUDIO_RECEIVER.get() {
        if let Ok(data) = receiver.try_recv() {
            let copy_len = buffer.len().min(data.len());
            buffer[..copy_len].copy_from_slice(&data[..copy_len]);
            return Ok(());
        }
    }
    Ok(())
}

fn build_stream_i16(
    device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<i16>>,
) -> Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[i16], _: &cpal::InputCallbackInfo| {
            let _ = sender.try_send(data.to_vec());
        },
        |err| error!("Audio stream error: {}", err),
        None,
    )?;
    Ok(stream)
}

fn build_stream_i8(
    device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<i16>>,
) -> Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[i8], _: &cpal::InputCallbackInfo| {
            let converted: Vec<i16> = data.iter().map(|&s| (s as i16) << 8).collect();
            let _ = sender.try_send(converted);
        },
        |err| error!("Audio stream error: {}", err),
        None,
    )?;
    Ok(stream)
}

fn build_stream_i32(
    device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<i16>>,
) -> Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[i32], _: &cpal::InputCallbackInfo| {
            let converted: Vec<i16> = data.iter().map(|&s| (s >> 16) as i16).collect();
            let _ = sender.try_send(converted);
        },
        |err| error!("Audio stream error: {}", err),
        None,
    )?;
    Ok(stream)
}

fn build_stream_i64(
    device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<i16>>,
) -> Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[i64], _: &cpal::InputCallbackInfo| {
            let converted: Vec<i16> = data.iter().map(|&s| (s >> 48) as i16).collect();
            let _ = sender.try_send(converted);
        },
        |err| error!("Audio stream error: {}", err),
        None,
    )?;
    Ok(stream)
}

fn build_stream_u8(
    device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<i16>>,
) -> Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[u8], _: &cpal::InputCallbackInfo| {
            let converted: Vec<i16> = data.iter().map(|&s| ((s as i16 - 128) << 8)).collect();
            let _ = sender.try_send(converted);
        },
        |err| error!("Audio stream error: {}", err),
        None,
    )?;
    Ok(stream)
}

fn build_stream_u16(
    device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<i16>>,
) -> Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[u16], _: &cpal::InputCallbackInfo| {
            let converted: Vec<i16> = data.iter().map(|&s| (s as i32 - 32768) as i16).collect();
            let _ = sender.try_send(converted);
        },
        |err| error!("Audio stream error: {}", err),
        None,
    )?;
    Ok(stream)
}

fn build_stream_u32(
    device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<i16>>,
) -> Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[u32], _: &cpal::InputCallbackInfo| {
            let converted: Vec<i16> = data
                .iter()
                .map(|&s| ((s >> 16) as i32 - 32768) as i16)
                .collect();
            let _ = sender.try_send(converted);
        },
        |err| error!("Audio stream error: {}", err),
        None,
    )?;
    Ok(stream)
}

fn build_stream_u64(
    device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<i16>>,
) -> Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[u64], _: &cpal::InputCallbackInfo| {
            let converted: Vec<i16> = data
                .iter()
                .map(|&s| ((s >> 48) as i32 - 32768) as i16)
                .collect();
            let _ = sender.try_send(converted);
        },
        |err| error!("Audio stream error: {}", err),
        None,
    )?;
    Ok(stream)
}

fn build_stream_f32(
    device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<i16>>,
) -> Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let converted: Vec<i16> = data
                .iter()
                .map(|&s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                .collect();
            let _ = sender.try_send(converted);
        },
        |err| error!("Audio stream error: {}", err),
        None,
    )?;
    Ok(stream)
}

fn build_stream_f64(
    device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<i16>>,
) -> Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[f64], _: &cpal::InputCallbackInfo| {
            let converted: Vec<i16> = data
                .iter()
                .map(|&s| (s.clamp(-1.0, 1.0) * i16::MAX as f64) as i16)
                .collect();
            let _ = sender.try_send(converted);
        },
        |err| error!("Audio stream error: {}", err),
        None,
    )?;
    Ok(stream)
}
