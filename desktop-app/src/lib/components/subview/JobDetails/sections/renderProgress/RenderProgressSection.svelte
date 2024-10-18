<script lang="ts">
  import { mapStatusToColor } from "$lib/data/jobs/transforms";
  import { JobStatus } from "$lib/data/jobState";

  const frameCount = 240;

  interface ITask {
    status: JobStatus;
    frameNumber: number;
    queuedAt: Date;
    startedAt?: Date;
    completedAt?: Date;
  }

  const startDate = new Date(Date.now() - 1000 * 60 * 60 * 8);

  const tasks: ITask[] = Array.from({ length: frameCount }).map((_, idx) => {
    const queuedAt = new Date(
      startDate.getTime() + 1000 * 60 * 50 * Math.random()
    );
    const startedAt = new Date(queuedAt.getTime() + 1000 * 60 * 5 * idx);

    const completedAt = new Date(startedAt.getTime() + 1000 * 60 * 20);

    let status = JobStatus.Pending;
    if (completedAt.getTime() < Date.now()) status = JobStatus.Succeeded;
    else if (startedAt.getTime() <= Date.now()) status = JobStatus.Running;

    return {
      status,
      frameNumber: idx + 1,
      queuedAt,
      startedAt: startedAt.getTime() > Date.now() ? undefined : startedAt,
      completedAt: completedAt.getTime() > Date.now() ? undefined : completedAt,
    };
  });

  const determineTaskTimes = (task: ITask) => {
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

<div
  class="overflow-visible collapse collapse-arrow border-base-300 bg-base-200 border"
>
  <input type="checkbox" class="peer" />
  <div class="collapse-title flex flex-col space-y-1 peer-checked:border-b-[1px] peer-checked:border-base-300">
    <div class="flex justify-between">
      <p class="text-md font-medium">Render Progress</p>

      <p>87%</p>
    </div>

    <progress class="progress progress-success h-1" value="40" max="100"></progress>
  </div>
  <div class="collapse-content flex flex-wrap space-x-2 space-y-2 px-2 mt-2">
    <div />
    {#each tasks as task}
      <div class="dropdown dropdown-left">
        <div
          tabindex="-1"
          class="h-4 w-4 bg-{mapStatusToColor(
            task.status
          )} rounded-sm cursor-pointer"
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
            <p
              class="text-info text-{mapStatusToColor(task.status)} capitalize"
            >
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
    {/each}
  </div>
</div>
