<script lang="ts">
  import "../app.css";
  import { invoke } from "@tauri-apps/api/core";
  import Navbar from "../lib/components/navigation/navbar/Navbar.svelte";
  import JobsTable from "$lib/components/display/tables/jobsTable/JobsTable.svelte";

  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="flex w-full">
  <Navbar />

  <div class="px-4 pt-8 flex flex-col flex-auto">
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
          <button class="join-item btn">1</button>
          <button class="join-item btn btn-active">2</button>
          <button class="join-item btn">3</button>
          <button class="join-item btn">4</button>
        </div>
    </div>
  </div>
</div>
