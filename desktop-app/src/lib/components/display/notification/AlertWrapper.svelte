<script lang="ts">
  import { getContext } from "svelte";
  import Alert from "./Alert.svelte";
  import { ContextKeys } from "$lib/state";
  import type { AlertItem } from "$lib/state/alerts";
  import type { Writable } from "svelte/store";
  import { scale } from "svelte/transition";

  // Only display a limited number at a time
  const MAX_ITEMS = 3;
  let alerts = getContext(ContextKeys.ALERTS) as Writable<AlertItem[]>;

  const removeAlert = (id: string) => {
    alerts.update(cur => cur.filter(item => item.id !== id))
  };

  let alertsLocal: AlertItem[] = [];

  $: alertsLocal = $alerts.slice(0, MAX_ITEMS);
</script>


{#each $alerts as alert}
<button onclick={() => removeAlert(alert.id)} transition:scale>
  <Alert msg={alert.msg} severity={alert.severity} onTimeout={() => removeAlert(alert.id)}/>
</button>
{/each}
