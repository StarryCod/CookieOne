<script lang="ts">
    import { goto } from "@roxi/routify";
    import { invoke } from "@tauri-apps/api/tauri";
    import {
        Button,
        Card,
        Center,
        Container,
        Progress,
        Stack,
        Text,
        Timeline,
        Title,
    } from "@svelteuidev/core";
    import { onDestroy } from "svelte";

    const REQUIRED_SAMPLES = 3;
    const RECORDING_DURATION_MS = 2000;

    let currentStep = 0;
    let recording = false;
    let recordedSamples: string[] = [];
    let trainingInProgress = false;
    let errorMessage = "";
    let infoMessage = "Нажмите кнопку и произнесите \"Cookie\"";

    let mediaStream: MediaStream | null = null;
    let mediaRecorder: MediaRecorder | null = null;
    let stopTimer: ReturnType<typeof setTimeout> | null = null;

    const stepsState: Array<"Ожидание" | "Запись" | "Готово" | "Ошибка"> = Array(REQUIRED_SAMPLES).fill(
        "Ожидание"
    );

    async function requestMicrophone(): Promise<MediaStream> {
        try {
            return await navigator.mediaDevices.getUserMedia({ audio: true });
        } catch (error) {
            throw new Error(
                "Не удалось получить доступ к микрофону. Разрешите доступ в настройках системы и попробуйте снова."
            );
        }
    }

    function resetRecordingState() {
        if (mediaRecorder && mediaRecorder.state !== "inactive") {
            mediaRecorder.stop();
        }

        if (mediaStream) {
            mediaStream.getTracks().forEach((track) => track.stop());
            mediaStream = null;
        }

        if (stopTimer) {
            clearTimeout(stopTimer);
            stopTimer = null;
        }

        mediaRecorder = null;
        recording = false;
    }

    async function startRecording(sampleIndex: number) {
        errorMessage = "";
        infoMessage = "Готовимся к записи...";

        try {
            mediaStream = await requestMicrophone();
            const chunks: BlobPart[] = [];

            mediaRecorder = new MediaRecorder(mediaStream, {
                mimeType: "audio/webm;codecs=opus",
            });

            mediaRecorder.ondataavailable = (event) => {
                if (event.data.size > 0) {
                    chunks.push(event.data);
                }
            };

            mediaRecorder.onstop = async () => {
                recording = false;
                infoMessage = "Обработка записи...";

                const blob = new Blob(chunks, { type: "audio/webm" });
                const arrayBuffer = await blob.arrayBuffer();
                const audioBytes = Array.from(new Uint8Array(arrayBuffer));

                try {
                    const samplePath = await invoke<string>("record_audio_sample", {
                        sampleIndex,
                        data: audioBytes,
                    });

                    recordedSamples = [...recordedSamples, samplePath];
                    stepsState[sampleIndex] = "Готово";
                    currentStep = sampleIndex + 1;
                    infoMessage =
                        currentStep < REQUIRED_SAMPLES
                            ? "Отлично! Нажмите, чтобы записать следующий образец."
                            : "Все образцы записаны. Можно переходить к обучению.";
                } catch (error) {
                    stepsState[sampleIndex] = "Ошибка";
                    errorMessage = `Ошибка сохранения записи: ${error}`;
                    console.error("record_audio_sample failed", error);
                } finally {
                    resetRecordingState();
                }
            };

            recording = true;
            stepsState[sampleIndex] = "Запись";
            infoMessage = "Скажите \"Cookie\"...";

            mediaRecorder.start();

            stopTimer = setTimeout(() => {
                if (mediaRecorder && mediaRecorder.state === "recording") {
                    mediaRecorder.stop();
                }
            }, RECORDING_DURATION_MS);
        } catch (error) {
            resetRecordingState();
            stepsState[sampleIndex] = "Ошибка";
            errorMessage = error instanceof Error ? error.message : String(error);
            console.error("startRecording failed", error);
        }
    }

    async function trainModel() {
        if (recordedSamples.length < REQUIRED_SAMPLES) {
            errorMessage = `Требуется ${REQUIRED_SAMPLES} записи, у вас ${recordedSamples.length}`;
            return;
        }

        trainingInProgress = true;
        errorMessage = "";
        infoMessage = "Обучение модели, пожалуйста подождите...";

        try {
            await invoke<string>("train_wakeword", {
                samplePaths: recordedSamples,
            });

            await invoke("set_onboarding_completed", { value: true });
            goto("/");
        } catch (error) {
            errorMessage = `Ошибка обучения: ${error}`;
            console.error("train_wakeword failed", error);
        } finally {
            trainingInProgress = false;
        }
    }

    function skipOnboarding() {
        invoke("set_onboarding_completed", { value: true }).then(() => goto("/"));
    }

    onDestroy(() => {
        resetRecordingState();
    });
</script>

<Container size="sm" style="margin-top: 2rem; margin-bottom: 2rem;">
    <Center>
        <Stack spacing="xl">
            <Title order={1} align="center">Добро пожаловать в Cookie! 🍪</Title>
            <Text align="center" color="dimmed">
                Мы настроим ассистента на ваш голос. Потребуется записать {REQUIRED_SAMPLES} образца,
                произнося слово «Cookie».
            </Text>

            <Timeline active={currentStep} bulletSize={24} lineWidth={3} radius="xl">
                {#each stepsState as status, index}
                    <Timeline.Item
                        title={`Образец ${index + 1}`}
                        bullet={index + 1}
                        color={status === "Готово" ? "teal" : status === "Ошибка" ? "red" : "blue"}
                    >
                        <Text size="sm" color={status === "Ошибка" ? "red" : "dimmed"}>
                            {status === "Запись"
                                ? "Идет запись..."
                                : status === "Готово"
                                ? "Записано"
                                : status === "Ошибка"
                                ? "Ошибка записи"
                                : "Ожидание"}
                        </Text>
                    </Timeline.Item>
                {/each}
            </Timeline>

            <Card shadow="sm" padding="lg">
                <Stack spacing="md">
                    <Title order={3} align="center">
                        {currentStep < REQUIRED_SAMPLES
                            ? `Шаг ${currentStep + 1} из ${REQUIRED_SAMPLES}`
                            : "Все образцы записаны"}
                    </Title>

                    <Text size="sm" align="center" color={errorMessage ? "red" : "dimmed"}>
                        {errorMessage || infoMessage}
                    </Text>

                    <Progress value={(currentStep / REQUIRED_SAMPLES) * 100} size="lg" />

                    {#if currentStep < REQUIRED_SAMPLES}
                        <Center>
                            <Button
                                size="xl"
                                radius="xl"
                                variant="gradient"
                                gradient={{ from: "orange", to: "red" }}
                                loading={recording}
                                disabled={recording}
                                on:click={() => startRecording(currentStep)}
                            >
                                {recording ? "Запись..." : `Записать образец ${currentStep + 1}`}
                            </Button>
                        </Center>
                    {:else}
                        <Center>
                            <Button
                                size="xl"
                                radius="xl"
                                variant="gradient"
                                gradient={{ from: "blue", to: "cyan" }}
                                loading={trainingInProgress}
                                on:click={trainModel}
                            >
                                {trainingInProgress ? "Обучаем модель..." : "Обучить модель"}
                            </Button>
                        </Center>
                    {/if}
                </Stack>
            </Card>

            <Center>
                <Button variant="subtle" on:click={skipOnboarding}>
                    Пропустить и использовать модель по умолчанию
                </Button>
            </Center>
        </Stack>
    </Center>
</Container>

<style>
    :global(body) {
        background: linear-gradient(135deg, #ffd6a5 0%, #ff8fab 100%);
    }

    :global(main) {
        min-height: calc(100vh - 120px);
    }
</style>
