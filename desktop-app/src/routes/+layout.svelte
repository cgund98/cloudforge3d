<script lang="ts">
  import ErrorWrapper from "$lib/components/display/notification/AlertWrapper.svelte";
  import Tray from "$lib/components/display/notification/Tray.svelte";
  import { ContextKeys } from "$lib/state";

  import { setContext } from "svelte";
  import { writable } from "svelte/store";

  const theme = "dark";

  // Context
  let alerts = writable([]);
  setContext(ContextKeys.ALERTS, alerts);
</script>

<div data-theme={theme}>
  <slot />

  <Tray />
</div>
