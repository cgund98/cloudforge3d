<script lang="ts">
  import ClockIcon from "$lib/components/display/icons/ClockIcon.svelte";
  import QuestionIcon from "$lib/components/display/icons/QuestionIcon.svelte";
  import Property from "$lib/components/display/Property.svelte";
  import { mapStatusToColor } from "$lib/data/jobs/transforms";
  import { JobStatus } from "$lib/data/jobState";
  import type { Task } from "$lib/data/tasks/task";
  import TaskBlock from "./TaskBlock.svelte";

  const frameCount = 240;

  const startDate = new Date(Date.now() - 1000 * 60 * 60 * 8);

  const tasks: Task[] = Array.from({ length: frameCount }).map((_, idx) => {
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
</script>

<div
  class="overflow-visible collapse collapse-arrow border-base-300 bg-base-200 border"
>
  <input type="checkbox" class="peer" />
  <div
    class="collapse-title flex flex-col space-y-1 peer-checked:border-b-[1px] peer-checked:border-base-300"
  >
    <div class="flex justify-between">
      <p class="text-md font-medium">Render Progress</p>

      <p>87%</p>
    </div>

    <progress class="progress progress-success h-1" value="40" max="100"
    ></progress>
  </div>
  <div class="collapse-content px-0">
    <div class="flex flex-wrap space-x-2 space-y-2 px-2 mt-2">
      <div />
      {#each tasks as task}
        <TaskBlock {task} />
      {/each}
    </div>

    <div class="px-4 mt-4 flex flex-col space-y-2">
      <Property label="Time Elapsed" value="2:04:23"><ClockIcon extraClass="stroke-neutral-content" /></Property>
      <Property label="Time Remaining" value="5:23:12"><QuestionIcon extraClass="stroke-neutral-content" /></Property>
    </div>
  </div>
</div>
