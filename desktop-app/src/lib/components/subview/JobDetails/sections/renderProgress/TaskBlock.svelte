<script lang="ts">
  import { formatDuration } from "$lib/data/date";
  import { mapTaskStatusToColor } from "$lib/data/jobs/transforms";
  import { TaskStatus, type Task } from "$lib/data/tasks/task";

  export let task: Task;


  const determineTaskTimes = (task: Task) => {
    if (task.completedAt !== undefined && task.startedAt !== undefined)
      return {
        label: "Render Time",
        value: formatDuration(
          task.completedAt.getTime() - task.startedAt.getTime()
        ),
      };

    if (task.startedAt !== undefined)
      return {
        label: "Render Time",
        value: formatDuration(Date.now() - task.startedAt.getTime()),
      };

    return {
      label: "Queue Time",
      value: task.queuedAt
        ? formatDuration(Date.now() - task.queuedAt.getTime())
        : "??",
    };
  };

  let time: {label: string, value: string} | undefined = undefined;

  $: time = determineTaskTimes(task);
</script>

<div class="dropdown dropdown-left">
  <div
    tabindex="-1"
    class="h-4 w-4 bg-{mapTaskStatusToColor(
      task.status
    )} rounded-sm cursor-pointer"
    role="button"
    onclick={() => time = determineTaskTimes(task)}
  />
  <div
    tabindex="-1"
    class="dropdown-content menu bg-base-200 border-[1px] border-base-300 rounded-md z-[1] w-52 p-2 px-1 shadow"
  >
    <div class="flex justify-between px-2">
      <p class="font-medium">Frame #</p>
      <p>{task.frameNumber}</p>
    </div>

    <div class="divider py-0 my-0" />

    <div class="flex justify-between px-2">
      <p class="font-medium">Status</p>
      <p class="text-info text-{mapTaskStatusToColor(task.status)} capitalize">
        {task.status}
      </p>
    </div>

    <div class="divider py-0 my-0" />

    <div class="flex justify-between px-2">
      <p class="font-medium">{time?.label}</p>
      <p>{time?.value}</p>
    </div>

    <div class="divider py-0 my-0" />

    <div class="flex justify-between">
      {#if task.status === TaskStatus.Failed || task.status === TaskStatus.Succeeded}
        <button class="btn btn-ghost btn-xs">View Logs</button>
      {:else}
        <div></div>
      {/if}

      {#if task.status === TaskStatus.Failed}
        <button class="text-secondary btn btn-ghost btn-xs">Retry</button>
      {:else}
        <div></div>
      {/if}
    </div>
  </div>
</div>
