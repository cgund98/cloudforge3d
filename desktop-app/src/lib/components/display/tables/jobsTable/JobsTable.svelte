<script lang="ts">
  import { mapStatusToColor } from "$lib/data/jobs/transforms";
  import { JobStatus } from "$lib/data/jobState";
  import { fade, fly, scale, slide } from "svelte/transition";
  import OpenIcon from "../../icons/OpenIcon.svelte";
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import { ContextKeys } from "$lib/state";

  const selectedJobId = getContext(ContextKeys.SELECTED_JOB_ID) as Writable<string>;

  interface Row {
    id: string;
    name: string;
    status: JobStatus;
    createdAt: Date;
  }

  export let rows: Row[] = [];

  // Transformations
  const formatDate = (date: Date): string => {
    const now = new Date()

    const timeOfDay = `${(date.getHours() + 1) % 13}:${String(date.getMinutes()).padStart(2, '0')} ${date.getHours() ? "PM" : "AM" }`

    if (date.toDateString() === now.toDateString()) return `Today, ${timeOfDay}`

    const weekAgo = new Date(Date.now() - 1000 * 60 * 60 * 24 * 7)
    const dayOfWeek = date.toLocaleString('en-us', {  weekday: 'long' })
    if (date.toDateString() !== now.toDateString() && date > weekAgo) return `${dayOfWeek}, ${timeOfDay}`

    const month = date.toLocaleDateString('en-us', { month: 'long'})

    return `${month} ${date.getDate()}`
  }
</script>

<div class="">
  <table class="table table-auto w-full transition">
    <!-- head -->
    <thead>
      <tr>
        <th>Name</th>
        <th class="text-right">Status</th>
        <th class="text-right">Created At</th>
        <th class="text-right"></th>
      </tr>
    </thead>

    <!-- body -->
    <tbody>
      {#each rows as row (row.id)}
        <tr>
          <td>{row.name}</td>
          <td class="text-right"><div class="capitalize badge badge-outline badge-{mapStatusToColor(row.status)}">{row.status}</div></td>
          <td class="text-right">{formatDate(row.createdAt)}</td>
          <td class="text-right">
            <button class="btn btn-ghost btn-sm btn-square" onclick={() => selectedJobId.set(row.id)}>
              <OpenIcon />
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
