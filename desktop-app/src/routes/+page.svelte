<script lang="ts">
  import "../app.css";
  import { invoke } from "@tauri-apps/api/core";
  import Navbar from "../lib/components/navigation/navbar/Navbar.svelte";
  import JobsTable from "$lib/components/display/tables/jobsTable/JobsTable.svelte";
  import JobsList from "$lib/components/subview/JobsList/JobsList.svelte";
  import JobDetails from "$lib/components/subview/JobDetails/JobDetails.svelte";

  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="flex w-full">
  <Navbar />

  <JobsList />

  <JobDetails />
</div>
