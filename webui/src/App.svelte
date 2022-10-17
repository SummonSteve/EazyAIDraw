<script>
  import Sidebar from './lib/components/Sidebar.svelte';
  import Main from './lib/components/Main.svelte';
  import Notification from './lib/components/Notification.svelte';
  import { onMount } from 'svelte';
  import { WsHandler } from './lib/scripts/main';

  onMount(async () => {
    //new WsHandler();
  });
</script>

<div class="">
  <Main />
  <Sidebar />
  <Notification />
</div>
