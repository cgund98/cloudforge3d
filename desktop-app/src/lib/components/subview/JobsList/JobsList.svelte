<script lang="ts">
  import JobsTable from "$lib/components/display/tables/jobsTable/JobsTable.svelte";
  import { parseProtoDate } from "$lib/data/date";
  import { createJob } from "$lib/data/jobs/commands/createJob";
  import type { ListJobsItem } from "$lib/data/jobs/commands/listJobs";
  import { listJobs } from "$lib/data/jobs/commands/listJobs";
  import { EventName, type JobStatusUpdateEvent } from "$lib/data/jobs/events";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy } from "svelte";

  let page: number = $state(0);
  let pageCount = 0;
  const pageSize = 10;

  let jobs: ListJobsItem[] = $state([]);
  let total: number = $state(0);

  const fetch = () => {
    const offset = page * pageSize;
    const limit = pageSize;
    listJobs({ offset, limit })
      .then((res) => {
        jobs = res.jobs;
        total = res.total;
        pageCount = Math.ceil(total / pageSize);
      })
      .catch(console.error);
  };

  // Start listening for status events
  let unMountFn = () => {};
  const init = async () => {
    fetch();
    const unlisten = await listen<string>(
      EventName.JOB_STATUS_UPDATE,
      (e) => {
        const event = JSON.parse(e.payload) as JobStatusUpdateEvent;
        const found = jobs.find(j => j.id === event.jobId)
        if (jobs.length < pageCount ||  found !== undefined) fetch()
      }
    );
    unMountFn = unlisten;
  };

  init();

  onDestroy(() => {
    unMountFn();
  });

</script>

<div class="px-4 pt-8 flex flex-col flex-1">
  <div class="flex w-full justify-between">
    <div class="prose mb-2">
      <h2 class="mb-0">Jobs</h2>
    </div>

    <div>
      <button
        class="join-item btn btn-sm btn-primary"
        onclick={() => {
          createJob().then(fetch).catch(console.error);
        }}>Create Job</button
      >
    </div>
  </div>

  <JobsTable
    rows={jobs.map((job) => ({
      ...job,
      createdAt: parseProtoDate(job.createdAt),
    }))}
  />

  {#if total > 0}
    <div
      class="flex w-full justify-between items-center py-2 px-4 border-t-[1px] border-base-200"
    >
      <div>
        <p class="text-sm">
          Showing <span class="font-semibold"
            >{1 + page * pageSize}-{page * pageSize + jobs.length}</span
          >
          of
          <span class="font-semibold">{total}</span> jobs.
        </p>
      </div>

      <div class="join">
        {#if total > pageSize}
          {#if page > 0}
            <button class="join-item btn" onclick={() => {
              page--;
              fetch();
            }}>«</button>
          {/if}
          <button class="join-item btn">Page {page + 1}</button>
          {#if page < pageCount - 1}
            <button class="join-item btn" onclick={() => {
              page++;
              fetch();
            }}>»</button>
          {/if}
        {/if}
      </div>
    </div>
    {:else}
    <p class="px-4 py-2">No jobs created yet.</p>
  {/if}
</div>
