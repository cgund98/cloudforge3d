<script lang="ts">
  import FolderDownloadIcon from "$lib/components/display/icons/FolderDownloadIcon.svelte";
  import Property from "$lib/components/display/Property.svelte";
  import PathInput from "$lib/components/input/PathInput.svelte";

  let checked = false;
</script>

<div class="flex flex-col border-base-300 bg-base-200 border px-4 py-1 pt-4 rounded-2xl space-y-3 overflow-hidden">
  <div class="">
    <div class="flex justify-between flex-row">
      <p class="text-md font-medium">Downloads</p>
      <button class="btn btn-primary btn-sm" on:click={(event) => {
        event.preventDefault()
        checked = false;
      }}>Download Frames</button>
    </div>
  </div>
  <div class="">
    <div class="flex w-full flex-col space-y-3">
      <PathInput
        label="Download Folder"
        value="/home/callum/Downloads/cowboy_lookdev/render"
      >
        <FolderDownloadIcon />
      </PathInput>

      <div class="flex space-x-3 justify-end w-full"></div>
    </div>
  </div>
</div>
