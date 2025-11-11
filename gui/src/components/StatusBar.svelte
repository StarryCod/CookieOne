<script lang="ts">
  export let status: string = "Готово";
  export let phrase: string = "";

  function getColor() {
    if (status === "Слушает") return "#3a7ca5";
    if (status === "Распознает") return "#e0e0e0";
    if (status === "Выполняет команду") return "#3fb950";
    return "#666";
  }
</script>

<div class="status-bar">
  <div class="status-indicator" style={`background-color: ${getColor()};`}></div>
  <div class="status-text">
    <span class="status-label">{status}</span>
    {#if phrase}
      <span class="status-phrase">{phrase}</span>
    {/if}
  </div>
</div>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    gap: 12px;
    background: #2a2a2a;
    border: 1px solid #3a3a3a;
    border-radius: 12px;
    padding: 12px 16px;
    color: #e0e0e0;
  }

  .status-indicator {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    box-shadow: 0 0 12px rgba(58, 124, 165, 0.7);
  }

  .status-text {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .status-label {
    font-size: 16px;
    font-weight: 600;
    color: #e0e0e0;
  }

  .status-phrase {
    font-size: 13px;
    color: #a0a0a0;
  }

  @media (max-width: 768px) {
    .status-bar {
      width: 100%;
    }
  }
</style>
