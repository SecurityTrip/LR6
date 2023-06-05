<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/tauri';

  let timing1;
  let connect1;

  let timing2;
  let connect2;

  let timing3;
  let connect3;

  let timing4;
  let connect4;

  let time1;
  let time2;
  let time3;
  let time4;

  let status1 = "Disconnected";
  let status2 = "Disconnected";
  let status3 = "Disconnected";
  let status4 = "Disconnected";
  
  async function initializeApp() {
    await invoke('initialize');
  }

  onMount(() => {
  initializeApp();

  timing1 = listen('updateConnectionTime1', (event) => {
    time1 = event.payload;
  });
  connect1 = listen('connectionSuccess1', (event) => {
    status1 = "Connected";
  });


  timing2 = listen('updateConnectionTime2', (event) => {
    time2 = event.payload;
  });
  connect2 = listen('connectionSuccess2', (event) => {
    status2 = "Connected";
  });

  timing3 = listen('updateConnectionTime3', (event) => {
    time3 = event.payload;
  });
  connect3 = listen('connectionSuccess3', (event) => {
    status3 = "Connected";
  });


  timing4 = listen('updateConnectionTime4', (event) => {
    time4 = event.payload;
  });
  connect4 = listen('connectionSuccess4', (event) => {
    status4 = "Connected";
  });
});


  onDestroy(() => {
    if (timing1) {
      timing1.close();
    }
    if (timing2) {
      timing2.close();
    }
    if (timing3) {
      timing3.close();
    }
    if (timing4) {
      timing4.close();
    }
  });
</script>

<main class="grid">
  <div>
    <h1>Client #1</h1>
    <h2>Connection status: {status1}</h2>
    <h2>Server waiting time: {time1} seconds</h2>
  </div>

  <div>
    <h1>Client #2</h1>
    <h2>Connection status: {status2}</h2>
    <h2>Server waiting time: {time2} seconds</h2>
  </div>

  <div>
    <h1>Client #3</h1>
    <h2>Connection status: {status3}</h2>
    <h2>Server waiting time: {time3} seconds</h2>
  </div>

  <div>
    <h1>Client#4</h1>
    <h2>Connection status: {status4}</h2>
    <h2>Server waiting time: {time4} seconds</h2>
  </div>
  
</main>
