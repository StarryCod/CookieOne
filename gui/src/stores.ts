import { invoke } from "@tauri-apps/api/tauri"
import { writable, derived } from 'svelte/store'

// Listening state
export const is_listening = writable(false);
let is_listening__val: boolean;
is_listening.subscribe(value => {
  is_listening__val = value;
});
export function isListening() {return is_listening__val}

// Recognized text
export const recognized_text = writable("");

// Current assistant phrase
export const current_phrase = writable("");

// Selected backend (Vosk / Gemini)
export const selected_backend = writable("Vosk");

// API key status
export const api_key_set = writable(false);

// Logs array
export const logs = writable<string[]>([]);

// Add log helper
export function addLog(message: string) {
  logs.update(l => {
    const newLogs = [...l, `[${new Date().toLocaleTimeString()}] ${message}`];
    return newLogs.slice(-50); // Keep last 50 logs
  });
}

// Assistant voice
export const assistant_voice = writable("");

(async () => {
  assistant_voice.set(await invoke("db_read", {key: "assistant_voice"}));
})().catch(err => {
    console.error(err);
});

(async () => {
  try {
    const configJson = await invoke<string>("get_config");
    const config = JSON.parse(configJson || "{}");
    selected_backend.set(config.speech_to_text_engine?.toString() === "Gemini" ? "Gemini" : "Vosk");
    api_key_set.set(Boolean(config.api_keys?.gemini));
  } catch (error) {
    console.error(error);
  }
})();

// Status (Готово, Слушает, Распознает)
export const status = writable("Готово");

// etc
export let tg_official_link = "";
export let feedback_link = "";
export let github_repository_link = "";
export let log_file_path = "";

(async () => {
  tg_official_link = await invoke("get_tg_official_link")
  feedback_link = await invoke("get_feedback_link")
  github_repository_link = await invoke("get_repository_link")
  log_file_path = await invoke("get_log_file_path")
})().catch(err => {
    console.error(err);
});