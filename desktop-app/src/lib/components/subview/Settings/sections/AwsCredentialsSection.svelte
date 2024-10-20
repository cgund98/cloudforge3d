<script>

  import EditIcon from "$lib/components/display/icons/EditIcon.svelte";

</script>
<div class="flex flex-col space-y-0">
  <div class="flex flex-row space-x-4">
    <div class="prose">
      <h3 class="">AWS Credentials</h3>
    </div>


    <div class="tooltip tooltip-right" data-tip="Edit">
      <button class="btn btn-ghost btn-sm btn-square">
        <EditIcon extraClass="size-5" />
      </button>
    </div>
  </div>

  <label class="form-control w-full max-w-xs">
    <div class="label">
      <span class="label-text">Access Key ID</span>
    </div>
    <input
      type="text"
      placeholder="Type here"
      readonly
      class="input input-bordered w-full max-w-xs"
    />
  </label>

  <label class="form-control w-full max-w-xs">
    <div class="label">
      <span class="label-text">Secret Access Key</span>
    </div>
    <input
      type="text"
      placeholder="Type here"
      readonly
      class="input input-bordered w-full max-w-xs"
    />
  </label>

  <div class="flex flex-row space-x-2 pt-4">
    <button class="btn btn-primary">Save Changes</button>
    <button class="btn btn-neutral">Reset Changes</button>
  </div>
</div>
