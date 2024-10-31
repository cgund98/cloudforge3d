<script lang="ts">
  import FolderDownloadIcon from "$lib/components/display/icons/FolderDownloadIcon.svelte";
  import PathInput from "$lib/components/input/PathInput.svelte";
  import { downloadFrames } from "$lib/data/jobs/commands/downloadFrames";
  import { updateJob } from "$lib/data/jobs/commands/updateJob";
  import type { Job } from "$lib/data/jobs/job";
  import { ContextKeys } from "$lib/state";
  import type { AlertItem } from "$lib/state/alerts";
  import { genSuccess } from "$lib/state/alerts/factory";
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";

  let { job }: { job: Job | undefined } = $props();

  const selectedJobId = getContext(
    ContextKeys.SELECTED_JOB_ID
  ) as Writable<string>;
  let alerts = getContext(ContextKeys.ALERTS) as Writable<AlertItem[]>;
</script>

{#if job !== undefined}
<div
  class="flex flex-col border-base-300 bg-base-200 border px-4 py-1 pt-4 rounded-2xl space-y-3 overflow-visible"
>
  <div class="">
    <div class="flex justify-between flex-row">
      <p class="text-md font-medium">Downloads</p>
      <button
        class="btn btn-primary btn-sm"
        onclick={() => {
          downloadFrames(job.id)
            .then((res) => {
              const newAlert = genSuccess(
                `Downloaded ${res.downloadedCount} frames.`
              );
              alerts.update((cur) => [...cur, newAlert]);
            })
            .catch(console.error);
        }}>Download Frames</button
      >
    </div>
  </div>
  <div class="">
    <div class="flex w-full flex-col space-y-3">
      <PathInput
        label="Download Folder"
        value={job.downloadPath}
        handle={(newPath) => {
          updateJob({ jobId: job.id, downloadPath: newPath })
            .catch(console.error);
        }}
      >
        <FolderDownloadIcon />
      </PathInput>

      <div class="flex space-x-3 justify-end w-full"></div>
    </div>
  </div>
</div>
{/if}