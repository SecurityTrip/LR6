<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/tauri';

  let messages = [];

  async function initializeApp() {
    await invoke('initialize');
  }

  onMount(() => {
    initializeApp();

    const unsubscribe = listen('msgs', (event) => {
      messages = [...messages, event.payload];
    });
  });

  function saveLogsToFile() {
    const logs = messages.join('\n');
    const blob = new Blob([logs], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = 'logs.txt';
    link.click();
    URL.revokeObjectURL(url);
  }
</script>

<main class="container">
  <h2>Логи</h2>
  <div class="log-container">
    <ul class="log-list">
      {#each messages as message}
        <li class="log-item">{message}</li>
      {/each}
    </ul>
  </div>
  <button on:click={saveLogsToFile}>Сохранить</button>
</main>

<style>
  .log-container {
    max-height: 300px;
    overflow-y: auto;
    border: 1px solid #ccc;
    padding: 10px;
  }

  .log-list {
    list-style: none;
    padding: 0;
  }

  .log-item {
    padding: 5px;
    margin-bottom: 5px;
    background-color: #f2f2f2;
    border-radius: 4px;
    color: black;
  }
</style>
