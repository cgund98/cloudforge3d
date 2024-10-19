<script lang="ts">
  import { mapStatusToColor } from "$lib/data/jobs/transforms";
  import type { Task } from "$lib/data/tasks/task";

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
      value: formatDuration(Date.now() - task.queuedAt.getTime()),
    };
  };

  const formatDuration = (duration: number) => {
    const seconds = Math.floor((duration / 1000) % 60);
    const minutes = Math.floor((duration / (1000 * 60)) % 60);
    const hours = Math.floor((duration / (1000 * 60 * 60)) % 24);

    const paddedHours = hours < 10 ? "0" + hours : hours;
    const paddedMinutes = minutes < 10 ? "0" + minutes : minutes;
    const paddedSeconds = seconds < 10 ? "0" + seconds : seconds;
    return `${paddedHours}:${paddedMinutes}:${paddedSeconds}`;
  };
</script>

<div class="dropdown dropdown-left">
  <div
    tabindex="-1"
    class="h-4 w-4 bg-{mapStatusToColor(task.status)} rounded-sm cursor-pointer"
    role="button"
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
      <p class="text-info text-{mapStatusToColor(task.status)} capitalize">
        {task.status}
      </p>
    </div>

    <div class="divider py-0 my-0" />

    <div class="flex justify-between px-2">
      <p class="font-medium">{determineTaskTimes(task).label}</p>
      <p>{determineTaskTimes(task).value}</p>
    </div>

    <div class="divider py-0 my-0" />

    <div class="flex justify-between">
      <button class="btn btn-ghost btn-xs">View Logs</button>
      <button class="text-secondary btn btn-ghost btn-xs">Retry</button>
    </div>
  </div>
</div>
