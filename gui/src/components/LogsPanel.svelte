<script lang="ts">
  import { Text, Card, Stack } from '@svelteuidev/core';
  import { logs } from '@/stores';

  let logsArray: string[] = [];

  logs.subscribe(value => {
    logsArray = value;
  });
</script>

<Card style="background-color: #2a2a2a; border: 1px solid #3a3a3a; padding: 16px;">
  <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
    <Text size="lg" weight={600} style="color: #3a7ca5;">Логи распознавания</Text>
    <Text size="xs" style="color: #666;">{logsArray.length} записей</Text>
  </div>
  
  <div class="logs-container">
    <Stack spacing="xs">
      {#each logsArray.slice(-20).reverse() as log}
        <div class="log-entry">
          <Text size="xs" style="color: #b0b0b0; font-family: 'Consolas', 'Monaco', monospace;">
            {log}
          </Text>
        </div>
      {/each}
      {#if logsArray.length === 0}
        <Text size="sm" style="color: #666; text-align: center; padding: 20px 0;">
          Логов пока нет. Начните работу с ассистентом.
        </Text>
      {/if}
    </Stack>
  </div>
</Card>

<style>
  .logs-container {
    max-height: 300px;
    overflow-y: auto;
    padding: 8px;
    background: #1a1a1a;
    border-radius: 8px;
  }

  .log-entry {
    padding: 6px 8px;
    border-bottom: 1px solid #333;
    transition: background-color 0.2s;
  }

  .log-entry:hover {
    background-color: #252525;
  }

  .log-entry:last-child {
    border-bottom: none;
  }

  .logs-container::-webkit-scrollbar {
    width: 8px;
  }

  .logs-container::-webkit-scrollbar-track {
    background: #1a1a1a;
    border-radius: 4px;
  }

  .logs-container::-webkit-scrollbar-thumb {
    background: #3a7ca5;
    border-radius: 4px;
  }

  .logs-container::-webkit-scrollbar-thumb:hover {
    background: #4a9cc5;
  }
</style>
