<script>

  import RefreshIcon from "$lib/components/display/icons/RefreshIcon.svelte";
  import SignalIcon from "$lib/components/display/icons/SignalIcon.svelte";
  import Property from "$lib/components/display/Property.svelte";

</script>
<div class="flex flex-col space-y-0">
    <div class="flex flex-row space-x-4">
      <div class="prose">
        <h3 class="">Queue Status</h3>
      </div>


      <div class="tooltip tooltip-right" data-tip="Refresh Status">
        <button class="btn btn-ghost btn-sm btn-square">
          <RefreshIcon extraClass="size-5" />
        </button>
      </div>
    </div>

    <div class="max-w-xs">
    <Property label="Status" value="Ready">
        <SignalIcon />
    </Property>
</div>
    </div>
