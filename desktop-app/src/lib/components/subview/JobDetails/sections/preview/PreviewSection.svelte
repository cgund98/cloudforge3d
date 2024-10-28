
<script lang="ts">
  import PauseIcon from "$lib/components/display/icons/PauseIcon.svelte";
  import PlayIcon from "$lib/components/display/icons/PlayIcon.svelte";
import type { Job } from "$lib/data/jobs/job";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { appDataDir, join } from "@tauri-apps/api/path";


  let {job}: {job: Job} = $props();

  let gifPath = $state("");
  let imagePath = $state("");
  let isSequence = $state(false);
  let paused = $state(true);

  const generatePaths = async (job: Job) => {
    const appDataDirPath = await appDataDir();
    gifPath = convertFileSrc(await join(appDataDirPath, `thumbnails/jobs/${job.id}/thumbnail.gif`));

    const paddedFrameNumber = String(job.frameStart).padStart(4, '0');
    imagePath = convertFileSrc(await join(appDataDirPath, `thumbnails/jobs/${job.id}/frame${paddedFrameNumber}.preview.webp`));

    isSequence = job.frameCount !== 1;
  };

  let genProm = $derived(generatePaths(job));
  let showGif = $derived(isSequence && gifPath !== "" && !paused);
</script>

{#await genProm}
{:then}
<div class="collapse collapse-arrow border-base-300 bg-base-200 border">
  <input type="checkbox" class="peer" checked />
  <div class="collapse-title">
    <p class="text-md font-medium">Preview</p>
  </div>
  <div class="collapse-content relative">
    <div class="flex w-full justify-around">
      <img
        src={gifPath}
        alt="Preview"
        class="max-h-[500px] max-w-full rounded overflow-hidden {showGif ? '' : 'hidden'}"
      />
      <img
        src={imagePath}
        alt="Preview"
        class="max-h-[500px] max-w-full rounded overflow-hidden {showGif ? 'hidden' : ''}"
      />
    </div>

    {#if isSequence && gifPath !== ""}
    <div class="absolute flex justify-around left-0 top-0 w-full h-full bg-base-200 bg-opacity-0 z-50 opacity-0 hover:opacity-100 ease-in-out">
      <div class="flex flex-col justify-around h-full">
        {#if paused}
        <button class="btn btn-ghost" onclick={() => paused = !paused}>
          <PlayIcon></PlayIcon>
        </button>
        {:else}
        <button class="btn btn-ghost" onclick={() => paused = !paused}>
          <PauseIcon></PauseIcon>
        </button>
        {/if}
      </div>
    </div>
    {/if}
  </div>
</div>
{/await}
