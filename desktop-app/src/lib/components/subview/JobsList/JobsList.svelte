<script>

  import JobsTable from "$lib/components/display/tables/jobsTable/JobsTable.svelte";

</script>
<div class="px-4 pt-8 flex flex-col flex-1">
    <div class="prose mb-2">
      <h2 class="mb-0">Jobs</h2>
    </div>

      <JobsTable />

      <div
        class="flex w-full justify-between items-center py-2 px-4 border-t-[1px] border-base-200"
      >
        <div>
          <p class="text-sm">
            Showing <span class="font-semibold">1-4</span> of
            <span class="font-semibold">4</span> jobs.
          </p>
        </div>

        <div class="join">
          <button class="join-item btn btn-sm">1</button>
          <button class="join-item btn btn-active btn-sm">2</button>
          <button class="join-item btn btn-sm">3</button>
          <button class="join-item btn btn-sm">4</button>
        </div>
    </div>
  </div>
