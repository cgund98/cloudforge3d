<script>
  import CogIcon from "$lib/components/display/icons/CogIcon.svelte";
  import LogoIcon from "$lib/components/display/icons/LogoIcon.svelte";

</script>

<nav class="h-[100vh] border-r-[1px] border-base-200 px-1 py-2 flex flex-col justify-between">

    <a href="/" class="btn btn-ghost btn-square">
        <LogoIcon />
    </a>

    <a href="/settings" class="btn btn-ghost btn-square">
      <CogIcon />
  </a>
</nav>
