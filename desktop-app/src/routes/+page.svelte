<script lang="ts">
  import "../app.css";
  import Navbar from "../lib/components/navigation/navbar/Navbar.svelte";
  import JobsList from "$lib/components/subview/JobsList/JobsList.svelte";
  import JobDetails from "$lib/components/subview/JobDetails/JobDetails.svelte";
  import { setContext } from "svelte";
  import { ContextKeys } from "$lib/state";
  import { writable } from "svelte/store";

  let selectedJobId = writable("");
  setContext(ContextKeys.SELECTED_JOB_ID, selectedJobId);

</script>

<div class="flex w-full overflow-y-hidden max-h-[100vh]" >
  <Navbar />

  <div class="flex w-full overflow-y-scroll">

  <JobsList />

  <JobDetails />
</div>
</div>
