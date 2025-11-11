<script lang="ts">
    import HDivider from "@/components/elements/HDivider.svelte"
    import Footer from "@/components/Footer.svelte"
    import CommandsView from "@/components/CommandsView.svelte"

    import { Text, Space, Alert  } from '@svelteuidev/core';
    import { InfoCircled } from 'radix-icons-svelte';

    import { tg_official_link } from "@/stores";
</script>

<Space h="xl" />

<div style="text-align: center; margin-bottom: 24px;">
    <Text size="xl" weight={700} style="color: #e0e0e0;">Библиотека команд</Text>
    <Space h="xs" />
    <Text size="sm" style="color: #a0a0a0;">Категории команд голосового ассистента</Text>
</div>

<Alert color="#3a7ca5" variant="light" icon={InfoCircled}>
    <Text size="sm" style="color: #e0e0e0;">
        Команды распознаются автоматически на основе фраз. Редактор команд в разработке.
    </Text>
</Alert>

<Space h="xl" />

<CommandsView />

<Space h="xl" />

<HDivider />
<Footer />