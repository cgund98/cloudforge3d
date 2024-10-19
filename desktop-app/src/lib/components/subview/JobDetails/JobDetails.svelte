<script lang="ts">
  import RefreshIcon from "$lib/components/display/icons/RefreshIcon.svelte";
  import StopIcon from "$lib/components/display/icons/StopIcon.svelte";
  import XIcon from "$lib/components/display/icons/XIcon.svelte";
  import { mapStatusToColor } from "$lib/data/jobs/transforms";
  import { JobStatus } from "$lib/data/jobState";
  import PreviewSection from "./sections/preview/PreviewSection.svelte";
  import PropertiesSection from "./sections/properties/PropertiesSection.svelte";
  import RenderProgressSection from "./sections/renderProgress/RenderProgressSection.svelte";

  const status: JobStatus = JobStatus.Pending;
</script>

<div class="px-4 pt-8 flex flex-col flex-1 border-l-[1px] border-base-200">
  <!-- Header -->
  <div class="mb-2 flex justify-between items-center">
    <div class="flex items-center space-x-2">
      <div class="tooltip tooltip-bottom capitalize" data-tip={status}>
        <div class="badge badge-{mapStatusToColor(status)} badge-xs"></div>
      </div>
      <div class="prose">
        <h2 class="mb-0 align-middle">first job</h2>
      </div>
    </div>

    <div class="flex space-x-3">
      <div class="tooltip tooltip-bottom" data-tip="Cancel Job">
        <button class="btn btn-ghost btn-sm btn-square">
          <StopIcon />
        </button>
      </div>
      <div class="tooltip tooltip-bottom" data-tip="Retry Failed Tasks">
        <button class="btn btn-ghost btn-sm btn-square">
          <RefreshIcon />
        </button>
      </div>

      <div class="tooltip tooltip-bottom" data-tip="Close">
        <button class="btn btn-ghost btn-sm btn-square">
          <XIcon />
        </button>
      </div>
    </div>
  </div>

  <!-- Sections -->

  <div class="flex flex-col space-y-4">
    <PreviewSection />

    <RenderProgressSection />

    <PropertiesSection />
  </div>
</div>
