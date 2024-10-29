<script lang="ts">
  import RefreshIcon from "$lib/components/display/icons/RefreshIcon.svelte";
  import StopIcon from "$lib/components/display/icons/StopIcon.svelte";
  import XIcon from "$lib/components/display/icons/XIcon.svelte";
  import { isCompleted, JobStatus, type Job } from "$lib/data/jobs/job";
  import { getContext, onDestroy } from "svelte";
  import ExportsSection from "./sections/exports/ExportsSection.svelte";
  import PreviewSection from "./sections/preview/PreviewSection.svelte";
  import PropertiesSection from "./sections/properties/PropertiesSection.svelte";
  import RenderProgressSection from "./sections/renderProgress/RenderProgressSection.svelte";
  import { ContextKeys } from "$lib/state";
  import type { Writable } from "svelte/store";
  import { getJob, type JobDetails } from "$lib/data/jobs/commands/getJob";
  import { listen } from "@tauri-apps/api/event";
  import { EventName, type JobStatusUpdateEvent } from "$lib/data/jobs/events";
  import { listTasks } from "$lib/data/tasks/commands/listTasks";
  import type { Task } from "$lib/data/tasks/task";
  import { mapJobStatusToColor } from "$lib/data/jobs/transforms";
  import { slide } from "svelte/transition";
  import { cancelJob } from "$lib/data/jobs/commands/cancelJob";
  import TrashIcon from "$lib/components/display/icons/TrashIcon.svelte";
  import { deleteJob } from "$lib/data/jobs/commands/deleteJob";

  let job: Job | null = null;
  let tasks: Task[] = [];

  const selectedJobId = getContext(
    ContextKeys.SELECTED_JOB_ID
  ) as Writable<string>;

  const fetch = (jobId: string) => {
    getJob({ jobId })
      .then((res) => {
        if (res === null || res === undefined) return;
        job = res;
      })
      .catch(console.error);

    listTasks({ jobId })
      .then((res) => (tasks = res))
      .catch(console.error);
  };

  selectedJobId.subscribe((jobId) => {
    const delay = job?.id ? 300 : 0;
    if (jobId !== job?.id) job = null;

    // Leave time for transition to occur
    setTimeout(() => {
      fetch(jobId);
    }, delay);
  });

  // Start listening for status events
  let unMountFn = () => {};
  const init = async () => {
    const unlisten = await listen<string>(EventName.JOB_STATUS_UPDATE, (e) => {
      const event = JSON.parse(e.payload) as JobStatusUpdateEvent;
      if (job?.id !== undefined && event.jobId === job?.id) fetch(job.id);
    });
    unMountFn = unlisten;
  };

  init();

  onDestroy(() => {
    unMountFn();
  });
</script>

{#if job !== null}
  <div
    class="px-4 pt-8 flex flex-col flex-1 border-l-[1px] border-base-200"
    transition:slide
  >
    <!-- Header -->
    <div class="mb-2 flex justify-between items-center">
      <div class="flex items-center space-x-2">
        <div class="tooltip tooltip-bottom capitalize" data-tip={job.status}>
          <div
            class="badge badge-{mapJobStatusToColor(job.status)} badge-xs"
          ></div>
        </div>
        <div class="prose">
          <h2 class="mb-0 align-middle">{job.name}</h2>
        </div>
      </div>

      <div class="flex space-x-3">
        {#if !isCompleted(job.status) && job.status !== JobStatus.Uploading}
          <div class="tooltip tooltip-bottom" data-tip="Cancel Job">
            <button
              class="btn btn-ghost btn-sm btn-square"
              onclick={() =>
                cancelJob(job?.id ?? "")
                  .then(fetch)
                  .catch(console.error)}
            >
              <StopIcon></StopIcon>
            </button>
          </div>
        {:else}
        <div class="tooltip tooltip-bottom" data-tip="Delete Job">
          <button
            class="btn btn-ghost btn-sm btn-square"
            onclick={() =>
              deleteJob(job?.id ?? "")
                .then(() => selectedJobId.set(""))
                .catch(console.error)}
          >
            <TrashIcon></TrashIcon>
          </button>
        </div>
        {/if}
        <!-- {#if job.status === JobStatus.Failed}
          <div class="tooltip tooltip-bottom" data-tip="Retry Failed Tasks">
            <button class="btn btn-ghost btn-sm btn-square">
              <RefreshIcon></RefreshIcon>
            </button>
          </div>
        {/if} -->

        <div class="tooltip tooltip-bottom" data-tip="Close">
          <button
            class="btn btn-ghost btn-sm btn-square"
            onclick={() => selectedJobId.set("")}
          >
            <XIcon />
          </button>
        </div>
      </div>
    </div>

    <!-- Sections -->

    <div class="flex flex-col space-y-4">
      <PreviewSection {job} />

      {#if job.status !== JobStatus.Uploading && job.status !== JobStatus.UploadFailed}
        <RenderProgressSection {job} {tasks} />
      {/if}

      <PropertiesSection {job} />

      <ExportsSection {job} />
    </div>
  </div>
{/if}
