<script lang="ts">
  import ErrorWrapper from "$lib/components/display/alert/AlertWrapper.svelte";
  import { ContextKeys } from "$lib/state";

  import { setContext } from "svelte";
  import { writable } from "svelte/store";

  const theme = "halloween";

  // Context
  let alerts = writable([]);
  setContext(ContextKeys.ALERTS, alerts);
</script>

<div data-theme={theme}>
  <slot />

  <ErrorWrapper />
</div>
