<script lang="ts">
  import { Modal, Button, Input, Text, Space, Alert, NativeSelect, Slider } from '@svelteuidev/core';
  import { CrossCircled } from 'radix-icons-svelte';
  import { invoke } from "@tauri-apps/api/tauri";
  import { selected_backend, api_key_set } from "@/stores";

  export let opened = false;

  let selectedBackend = "Vosk";
  let geminiApiKey = "";
  let selectedMicrophone = "";
  let wakeWordThreshold = 0.5;
  let availableMicrophones = [];
  let selectedWakeWordEngine = "rustpotter";

  selected_backend.subscribe(value => {
    selectedBackend = value;
  });

  async function loadSettings() {
    try {
      const configJson = await invoke<string>("get_config");
      const config = JSON.parse(configJson || "{}");
      
      selectedMicrophone = String(config.microphone ?? "");
      selectedWakeWordEngine = config.wake_word_engine ? config.wake_word_engine.toString().toLowerCase() : "rustpotter";
      geminiApiKey = config.api_keys?.gemini ?? "";
      wakeWordThreshold = config.wake_word_threshold ?? 0.5;
      selectedBackend = config.speech_to_text_engine?.toString() === "Gemini" ? "Gemini" : "Vosk";
      api_key_set.set(Boolean(geminiApiKey));

      const mics = await invoke<any>("pv_get_audio_devices");
      availableMicrophones = Object.entries(mics).map(([k, v]) => ({
        label: v as string,
        value: k
      }));

      selected_backend.set(selectedBackend);
    } catch (error) {
      console.error("Failed to load settings:", error);
    }
  }

  async function saveSettings() {
    try {
      await invoke("set_listening_device", {deviceIndex: parseInt(selectedMicrophone)});
      await invoke("db_write", {key: "selected_wake_word_engine", val: selectedWakeWordEngine});
      await invoke("db_write", {key: "wake_word_threshold", val: String(wakeWordThreshold)});
      await invoke("db_write", {key: "selected_backend", val: selectedBackend});
      
      if (selectedBackend === "Gemini") {
        await invoke("save_gemini_api_key", {key: geminiApiKey});
        api_key_set.set(geminiApiKey.trim().length > 0);
      }

      selected_backend.set(selectedBackend);
      opened = false;
      
      // Reload the page to apply changes (optional)
      window.location.reload();
    } catch (error) {
      console.error("Failed to save settings:", error);
    }
  }

  $: if (opened) {
    loadSettings();
  }
</script>

<Modal {opened} on:close={() => opened = false} title="Настройки" size="lg">
  <Space h="md" />

  <!-- Backend Selection -->
  <NativeSelect 
    data={[
      { label: 'Vosk (Офлайн)', value: 'Vosk' },
      { label: 'Gemini (Требует API ключ)', value: 'Gemini' }
    ]}
    label="Движок распознавания речи"
    description="Выберите STT backend для распознавания команд"
    variant="filled"
    bind:value={selectedBackend}
  />

  <Space h="md" />

  <!-- Gemini API Key Input -->
  {#if selectedBackend === "Gemini"}
    <Alert title="Внимание!" color="#d32f2f" variant="light" icon={CrossCircled}>
      <Text size="sm" style="color: #e0e0e0;">
        Для работы с Gemini требуется API ключ. Получите его на 
        <a href="https://makersuite.google.com/app/apikey" target="_blank" style="color: #3a7ca5;">
          makersuite.google.com
        </a>
      </Text>
    </Alert>
    <Space h="sm" />
    <Input 
      placeholder='Введите Gemini API ключ' 
      variant='filled' 
      bind:value={geminiApiKey}
      type="password"
    />
  {/if}

  <Space h="md" />

  <!-- Wake Word Engine -->
  <NativeSelect 
    data={[
      { label: 'Rustpotter (Рекомендуется)', value: 'rustpotter' },
      { label: 'Vosk (Медленный)', value: 'vosk' },
      { label: 'Picovoice Porcupine', value: 'picovoice' }
    ]}
    label="Движок активационной фразы (Wake Word)"
    description="Выберите метод распознавания активационной фразы"
    variant="filled"
    bind:value={selectedWakeWordEngine}
  />

  <Space h="md" />

  <!-- Microphone Selection -->
  <NativeSelect 
    data={availableMicrophones}
    label="Микрофон"
    description="Выберите устройство ввода для голосовых команд"
    variant="filled"
    bind:value={selectedMicrophone}
  />

  <Space h="md" />

  <!-- Wake Word Threshold -->
  <div>
    <Text size="sm" weight={600} style="color: #3a7ca5; margin-bottom: 8px;">
      Порог активации (0.1 - 1.0)
    </Text>
    <Slider 
      bind:value={wakeWordThreshold}
      min={0.1}
      max={1.0}
      step={0.05}
      color="#3a7ca5"
    />
    <Text size="xs" style="color: #a0a0a0; margin-top: 4px;">
      Текущее значение: {wakeWordThreshold.toFixed(2)}
    </Text>
  </div>

  <Space h="xl" />

  <div style="display: flex; gap: 12px;">
    <Button color="blue" fullSize on:click={saveSettings}>
      Сохранить
    </Button>
    <Button color="gray" variant="subtle" fullSize on:click={() => opened = false}>
      Отмена
    </Button>
  </div>
</Modal>

<style>
  :global(.svelteui-Modal-modal) {
    background-color: #2a2a2a !important;
    border: 1px solid #3a3a3a;
  }

  :global(.svelteui-Modal-title) {
    color: #e0e0e0 !important;
    font-weight: 600;
  }
</style>
