<script lang="ts">
  import RefreshIcon from "$lib/components/display/icons/RefreshIcon.svelte";
  import StopIcon from "$lib/components/display/icons/StopIcon.svelte";
  import XIcon from "$lib/components/display/icons/XIcon.svelte";
  import { mapStatusToColor } from "$lib/data/jobs/transforms";
  import { isCompleted, JobStatus } from "$lib/data/jobState";
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

  let status: JobStatus = JobStatus.Pending;

  let job: JobDetails | null = null;

  const selectedJobId = getContext(
    ContextKeys.SELECTED_JOB_ID
  ) as Writable<string>;

  const fetch = (jobId: string) =>
    getJob({ jobId }).then((res) => {
      if (res.job === null || res.job === undefined) {
        job = null;
        return;
      }

      job = res.job;
    });

  selectedJobId.subscribe((jobId) => fetch(jobId));

  // Start listening for status events
  let unMountFn = () => {};
  const init = async () => {
    const unlisten = await listen<string>(
      EventName.JOB_STATUS_UPDATE,
      (e) => {
        const event = JSON.parse(e.payload) as JobStatusUpdateEvent;
        if (job?.id !== null && event.jobId === job?.id) fetch(job.id);
      }
    );
    unMountFn = unlisten;
  };

  init();

  onDestroy(() => {
    unMountFn();
  });
</script>

{#if job !== null}
  <div class="px-4 pt-8 flex flex-col flex-1 border-l-[1px] border-base-200">
    <!-- Header -->
    <div class="mb-2 flex justify-between items-center">
      <div class="flex items-center space-x-2">
        <div class="tooltip tooltip-bottom capitalize" data-tip={job.status}>
          <div
            class="badge badge-{mapStatusToColor(job.status)} badge-xs"
          ></div>
        </div>
        <div class="prose">
          <h2 class="mb-0 align-middle">{job.name}</h2>
        </div>
      </div>

      <div class="flex space-x-3">
        {#if !isCompleted(job.status)}
          <div class="tooltip tooltip-bottom" data-tip="Cancel Job">
            <button class="btn btn-ghost btn-sm btn-square">
              <StopIcon />
            </button>
          </div>
        {/if}
        <div class="tooltip tooltip-bottom" data-tip="Retry Failed Tasks">
          <button class="btn btn-ghost btn-sm btn-square">
            <RefreshIcon />
          </button>
        </div>

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
      <PreviewSection />

      <RenderProgressSection />

      <PropertiesSection {job} />

      <ExportsSection />
    </div>
  </div>
{/if}
