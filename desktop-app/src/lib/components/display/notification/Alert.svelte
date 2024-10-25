<script lang="ts">
  import AlertErrorIcon from "../icons/alert/AlertErrorIcon.svelte";
  import AlertInfoIcon from "../icons/alert/AlertInfoIcon.svelte";
  import AlertSuccessIcon from "../icons/alert/AlertSuccessIcon.svelte";
  import AlertWarningIcon from "../icons/alert/AlertWarningIcon.svelte";

  export let msg: string;
  export let severity: string;
  export let timeout = 5000;
  export let onTimeout = () => {};

  setTimeout(onTimeout, timeout);
</script>

<div role="alert" class="alert alert-{severity}">
  {#if severity == "info"}
    <AlertInfoIcon />
  {:else if severity == "warning"}
    <AlertWarningIcon />
  {:else if severity == "error"}
    <AlertErrorIcon />
  {:else}
    <AlertSuccessIcon />
  {/if}
  <span>{msg}</span>
</div>
