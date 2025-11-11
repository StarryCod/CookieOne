mod config;
mod jarvis;
mod commands;
mod wakeword;
mod stt;
mod audio_pipeline;

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;

fn main() -> Result<()> {
    // Инициализируем логирование
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    log::info!("🍪 Cookie Voice Assistant v{}", env!("CARGO_PKG_VERSION"));
    log::info!("Запуск приложения...");
    
    // Загружаем конфигурацию
    let config_path = config::get_config_path()
        .context("Не удалось определить путь к конфигурации")?;
    
    log::info!("Путь к конфигурации: {}", config_path.display());
    
    let config = config::Config::load_or_default(&config_path)
        .context("Не удалось загрузить конфигурацию")?;
    
    log::info!("Конфигурация загружена");
    log::info!("  Wake-word порог: {}", config.wake_word_threshold);
    log::info!("  STT движок: {:?}", config.stt_backend);
    
    // Загружаем фразы JARVIS
    let app_dir = std::env::current_dir()?;
    let jarvis_phrases_path = app_dir.join(&config.jarvis_phrases);
    
    let jarvis_phrases = jarvis::JarvisPhrases::load_or_default(&jarvis_phrases_path)
        .context("Не удалось загрузить фразы JARVIS")?;
    
    log::info!("Фразы JARVIS загружены");
    
    // Загружаем библиотеку команд
    let commands_path = app_dir.join(&config.commands_path);
    
    let command_library = commands::CommandLibrary::load_or_default(&commands_path)
        .context("Не удалось загрузить библиотеку команд")?;
    
    log::info!("Загружено команд: {}", command_library.commands.len());
    
    // Инициализируем wake-word детектор
    let wakeword_path = app_dir.join(&config.wake_word_path);
    
    let mut wakeword_detector = wakeword::RustpotterDetector::new(
        &wakeword_path,
        config.wake_word_threshold,
    )
    .context("Не удалось инициализировать wake-word детектор")?;
    
    log::info!("Wake-word детектор инициализирован");
    
    // Инициализируем STT движок
    let mut stt_engine = stt::SttBackend::from_config(
        &config.stt_backend,
        config.gemini_api_key.clone(),
    )
    .context("Не удалось инициализировать STT движок")?;
    
    log::info!("STT движок инициализирован");
    
    // Инициализируем аудио-пайплайн
    let mut audio_pipeline = audio_pipeline::AudioPipeline::new(config.listening_device)
        .context("Не удалось инициализировать аудио-пайплайн")?;
    
    log::info!("Аудио-пайплайн инициализирован");
    
    // Запускаем основной цикл
    log::info!("🎤 Запуск прослушивания...");
    run_listening_loop(
        &mut audio_pipeline,
        &mut wakeword_detector,
        &mut stt_engine,
        &jarvis_phrases,
        &command_library,
    )?;
    
    Ok(())
}

/// Основной цикл прослушивания
fn run_listening_loop(
    audio_pipeline: &mut audio_pipeline::AudioPipeline,
    wakeword_detector: &mut wakeword::RustpotterDetector,
    stt_engine: &mut stt::SttBackend,
    jarvis_phrases: &jarvis::JarvisPhrases,
    command_library: &commands::CommandLibrary,
) -> Result<()> {
    // Запускаем захват аудио
    let audio_rx = audio_pipeline.start()
        .context("Не удалось запустить аудио-пайплайн")?;
    
    log::info!("✅ Слушаю wake-word...");
    
    let mut listening_for_command = false;
    let mut command_audio_buffer: Vec<i16> = Vec::new();
    let mut last_wake_time = std::time::Instant::now();
    
    // Основной цикл обработки аудио
    loop {
        match audio_rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(samples) => {
                if !listening_for_command {
                    // Режим ожидания wake-word
                    if let Some((name, score)) = wakeword_detector.process_samples(&samples) {
                        log::info!("🔔 Wake-word обнаружен: '{}' (score: {:.3})", name, score);
                        
                        // Произносим фразу активации
                        let wake_phrase = jarvis_phrases.get_random_wake();
                        log::info!("💬 {}", wake_phrase);
                        
                        // Переходим в режим прослушивания команды
                        listening_for_command = true;
                        command_audio_buffer.clear();
                        last_wake_time = std::time::Instant::now();
                    }
                } else {
                    // Режим прослушивания команды
                    command_audio_buffer.extend_from_slice(&samples);
                    
                    // Проверяем таймаут (15 секунд)
                    if last_wake_time.elapsed() > std::time::Duration::from_secs(15) {
                        log::info!("⏱️  Таймаут прослушивания команды");
                        
                        if !command_audio_buffer.is_empty() {
                            // Распознаем команду
                            process_command(
                                stt_engine,
                                &command_audio_buffer,
                                jarvis_phrases,
                                command_library,
                            );
                        }
                        
                        listening_for_command = false;
                        command_audio_buffer.clear();
                    }
                    
                    // Можно добавить детекцию тишины для завершения команды
                }
            }
            Err(crossbeam::channel::RecvTimeoutError::Timeout) => {
                // Таймаут - ничего не делаем, продолжаем цикл
                continue;
            }
            Err(e) => {
                log::error!("Ошибка получения аудио: {}", e);
                break;
            }
        }
    }
    
    audio_pipeline.stop();
    Ok(())
}

/// Обрабатывает распознанную команду
fn process_command(
    stt_engine: &mut stt::SttBackend,
    audio: &[i16],
    jarvis_phrases: &jarvis::JarvisPhrases,
    command_library: &commands::CommandLibrary,
) {
    log::info!("🎙️  Распознаю команду...");
    
    // Создаем runtime для async операций
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    let text = runtime.block_on(async {
        use stt::SpeechToText;
        stt_engine.transcribe(audio).await
    });
    
    match text {
        Ok(text) => {
            if text.is_empty() {
                log::info!("🔇 Ничего не распознано");
                return;
            }
            
            log::info!("📝 Распознано: '{}'", text);
            
            // Ищем команду в библиотеке
            if let Some(command) = command_library.find_by_text(&text) {
                log::info!("✨ Выполняю команду: {}", command.name);
                
                let ack_phrase = jarvis_phrases.get_random_ack();
                log::info!("💬 {}", ack_phrase);
                
                // Выполняем команду
                match execute_command(command) {
                    Ok(_) => {
                        let done_phrase = jarvis_phrases.get_random_done();
                        log::info!("💬 {}", done_phrase);
                    }
                    Err(e) => {
                        log::error!("❌ Ошибка выполнения команды: {}", e);
                        let error_phrase = jarvis_phrases.get_random_error();
                        log::info!("💬 {}", error_phrase);
                    }
                }
            } else {
                log::info!("❓ Команда не найдена в библиотеке");
                let error_phrase = jarvis_phrases.get_random_error();
                log::info!("💬 {}", error_phrase);
            }
        }
        Err(e) => {
            log::error!("❌ Ошибка распознавания: {}", e);
            let error_phrase = jarvis_phrases.get_random_error();
            log::info!("💬 {}", error_phrase);
        }
    }
}

/// Выполняет команду
fn execute_command(command: &commands::CommandDefinition) -> Result<()> {
    match &command.action {
        commands::CommandAction::RunProcess { command: cmd, args, working_dir } => {
            let mut process = std::process::Command::new(cmd);
            
            if !args.is_empty() {
                process.args(args);
            }
            
            if let Some(dir) = working_dir {
                process.current_dir(dir);
            }
            
            let output = process.output()
                .context("Не удалось запустить процесс")?;
            
            if output.status.success() {
                log::info!("✅ Процесс завершен успешно");
            } else {
                log::warn!("⚠️  Процесс завершен с ошибкой: {}", output.status);
            }
            
            Ok(())
        }
        commands::CommandAction::PlayAudio { file } => {
            log::info!("🔊 Воспроизведение аудио: {}", file);
            // TODO: Реализовать через rodio
            Ok(())
        }
        commands::CommandAction::RespondText { text } => {
            log::info!("💬 Ответ: {}", text);
            // TODO: Реализовать TTS
            Ok(())
        }
    }
}
