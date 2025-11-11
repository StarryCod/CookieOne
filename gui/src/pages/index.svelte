<script lang="ts">
    import ArcReactor from "@/components/elements/ArcReactor.svelte"
    import HDivider from "@/components/elements/HDivider.svelte"
    import Stats from "@/components/elements/Stats.svelte"
    import Footer from "@/components/Footer.svelte"

    import { Button, Text, Space, Card, Badge, Stack, Group  } from '@svelteuidev/core'
    import { Gear } from 'radix-icons-svelte'
    import { onMount, onDestroy } from 'svelte'
    import { startListening, stopListening } from "@/functions"

    import StatusBar from "@/components/StatusBar.svelte"
    import LogsPanel from "@/components/LogsPanel.svelte"
    import SettingsModal from "@/components/SettingsModal.svelte"

    import { 
        is_listening, 
        status, 
        current_phrase, 
        recognized_text,
        logs
    } from "@/stores"

    let is_listening__val: boolean = false;
    let status_val: string = "Готово";
    let current_phrase_val: string = "";
    let recognized_text_val: string = "";
    let logs_val: string[] = [];
    let settingsOpened = false;

    is_listening.subscribe(value => {
        is_listening__val = value;
    });

    status.subscribe(value => {
        status_val = value;
    });

    current_phrase.subscribe(value => {
        current_phrase_val = value;
    });

    recognized_text.subscribe(value => {
        recognized_text_val = value;
    });

    logs.subscribe(value => {
        logs_val = value;
    });

    onMount(async () => {
        document.body.classList.add('assist-page');
    });

    onDestroy(async () => {
        document.body.classList.remove('assist-page');
    });

    function toggleListening() {
        if (is_listening__val) {
            stopListening(null);
        } else {
            startListening();
        }
    }

    function getStatusColor() {
        if (status_val === "Слушает") return "blue";
        if (status_val === "Распознает") return "yellow";
        if (status_val === "Выполняет команду") return "green";
        return "gray";
    }
</script>

<HDivider />

<!-- Status Bar -->
<Group position="apart" style="margin: 20px 0;">
    <StatusBar status={status_val} phrase={current_phrase_val} />
    <Button leftIcon={Gear} color="gray" variant="light" on:click={() => settingsOpened = true}>
        Настройки
    </Button>
</Group>

<Space h="sm" />

<!-- Main Control Button -->
<div style="display: flex; justify-content: center; align-items: center; min-height: 300px;">
    <div style="text-align: center;">
        <ArcReactor />
        <Space h="xl" />
        <Button 
            color={is_listening__val ? "red" : "blue"} 
            size="xl" 
            radius="xl"
            on:click={toggleListening}
            style="min-width: 200px;"
        >
            {is_listening__val ? "Остановить" : "Слушать"}
        </Button>
    </div>
</div>

<!-- Current Phrase Display -->
{#if current_phrase_val}
<div style="text-align: center; margin: 20px 0;">
    <Text size="xl" color="#3a7ca5" weight={600}>{current_phrase_val}</Text>
</div>
{/if}

<!-- Recognized Text Display -->
{#if recognized_text_val}
<div style="text-align: center; margin: 20px 0;">
    <Text size="lg" color="#e0e0e0">Распознано: {recognized_text_val}</Text>
</div>
{/if}

<Space h="xl" />

<!-- Logs Panel -->
<LogsPanel />

<HDivider no_margin />
<Stats />
<Footer />

<!-- Settings Modal -->
<SettingsModal bind:opened={settingsOpened} />


<!-- 
<Title order={1}>This is h1 title</Title>
<Title order={1} variant='gradient' gradient={{from: 'blue', to: 'red', deg: 45}}>This is h1 title with a twist</Title>

<Menu>
  <Button slot="control" variant="gradient" gradient={{ from: 'blue', to: 'teal', deg: 50 }} radius="md" size="md">Toggle Menu</Button>
    <Menu.Label>Application</Menu.Label>
    <Menu.Item icon={Gear}>Settings</Menu.Item>
    <Menu.Item icon={ChatBubble}>Messages</Menu.Item>
    <Menu.Item icon={Camera}>Gallery</Menu.Item>
    <Menu.Item icon={MagnifyingGlass}>
        <svelte:fragment slot='rightSection'>
            <Text size="xs" color="dimmed">⌘K</Text>
        </svelte:fragment>
        Search
    </Menu.Item>

    <Divider />

    <Menu.Label>Danger zone</Menu.Label>
    <Menu.Item icon={Width}>Transfer my data</Menu.Item>
    <Menu.Item color="red" icon={Trash}>Delete my account</Menu.Item>
</Menu>

<Checkbox bind:checked={checked} label="I agree to sell my privacy" />
{checked}
{#if checked}
YEP!
{/if} -->