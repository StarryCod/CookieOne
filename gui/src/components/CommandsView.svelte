<script lang="ts">
  const categories = [
    {
      name: 'Медиа',
      description: 'Музыка, видео, управление медиаплеерами',
      icon: '/media/icons/media.svg'
    },
    {
      name: 'Автоматизация',
      description: 'Системные макросы, сценарии, процессы',
      icon: '/media/icons/automation.svg'
    },
    {
      name: 'Информация',
      description: 'Погода, время, новости, справка',
      icon: '/media/icons/info.svg'
    },
    {
      name: 'Система',
      description: 'Перезагрузка, выключение, статус ПК',
      icon: '/media/icons/system.svg'
    },
    {
      name: 'Кастомные команды',
      description: 'Ваши сценарии и кастомные действия',
      icon: '/media/icons/custom.svg'
    }
  ];
</script>

<div class="commands-view">
  {#each categories as category}
    <div class="command-card">
      <div class="icon-wrapper">
        <img src={category.icon} alt={category.name} />
      </div>
      <div class="info">
        <h3>{category.name}</h3>
        <p>{category.description}</p>
      </div>
    </div>
  {/each}
</div>

<style>
  .commands-view {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 16px;
    width: 100%;
  }

  .command-card {
    background: #2a2a2a;
    border: 1px solid #3a3a3a;
    border-radius: 16px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    transition: border-color 0.2s, transform 0.2s;
  }

  .command-card:hover {
    border-color: #3a7ca5;
    transform: translateY(-2px);
  }

  .icon-wrapper {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    background: #1a1a1a;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-wrapper img {
    width: 32px;
    height: 32px;
  }

  .info h3 {
    margin: 0;
    color: #e0e0e0;
    font-size: 18px;
  }

  .info p {
    margin: 0;
    color: #a0a0a0;
    font-size: 13px;
    line-height: 1.4;
  }

  @media (max-width: 768px) {
    .commands-view {
      grid-template-columns: 1fr;
    }
  }
</style>
