<script>
    import { onMount, onDestroy } from 'svelte'
    import { emit, listen } from '@tauri-apps/api/event'
    import { resolveResource } from '@tauri-apps/api/path'
    import {Howl, Howler} from 'howler';
    import { invoke } from '@tauri-apps/api/tauri';

    import { 
        assistant_voice, 
        status, 
        current_phrase, 
        recognized_text,
        addLog 
    } from "@/stores"

    let assistant_voice_val = "jarvis-og";
    assistant_voice.subscribe(value => {
        assistant_voice_val = value;
    });

    onMount(async () => {
        // Audio play event
        await listen('audio-play', async (event) => {
            let filename = 'sound/' + (assistant_voice_val == "" ? "jarvis-remake":assistant_voice_val) + '/' + event.payload['data'] + '.wav';
            await invoke("play_sound", {
                filename: filename,
                sleep: true
            });
        });

        // Wake word detected - assistant greeting
        await listen('wake-word-detected', (event) => {
            status.set("Слушает");
            current_phrase.set("Я вас слушаю, сэр.");
            addLog("Активационная фраза обнаружена");
            document.getElementById("arc-reactor")?.classList.add("active");
        });

        // Assistant greet
        await listen('assistant-greet', (event) => {
            status.set("Слушает");
            current_phrase.set("Я вас слушаю, сэр.");
            addLog("Ассистент приветствует");
            document.getElementById("arc-reactor")?.classList.add("active");
        });

        // Speech recognized
        await listen('speech-recognized', (event) => {
            status.set("Распознает");
            const text = event.payload['data'] || event.payload;
            recognized_text.set(text);
            addLog(`Распознано: ${text}`);
        });

        // Command executed
        await listen('command-executed', (event) => {
            const result = event.payload['data'] || event.payload;
            addLog(`Команда выполнена: ${result}`);
        });

        // Command start
        await listen('command-start', (event) => {
            status.set("Выполняет команду");
            addLog("Начало выполнения команды");
        });

        // Command in process
        await listen('command-in-process', (event) => {
            status.set("Выполняет команду");
        });

        // Command end
        await listen('command-end', (event) => {
            status.set("Готово");
            addLog("Команда завершена");
        });

        // Error occurred
        await listen('error-occurred', (event) => {
            const error = event.payload['data'] || event.payload;
            addLog(`Ошибка: ${error}`);
        });

        // Assistant waiting
        await listen('assistant-waiting', (event) => {
            status.set("Готово");
            current_phrase.set("");
            recognized_text.set("");
            addLog("Ассистент в режиме ожидания");
            document.getElementById("arc-reactor")?.classList.remove("active");
        });
    });
</script>