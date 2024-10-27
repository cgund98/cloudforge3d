<script lang="ts">
  import ClockIcon from "$lib/components/display/icons/ClockIcon.svelte";
  import QuestionIcon from "$lib/components/display/icons/QuestionIcon.svelte";
  import Property from "$lib/components/display/Property.svelte";
  import { TaskStatus, type Task } from "$lib/data/tasks/task";
  import { onMount } from "svelte";
  import TaskBlock from "./TaskBlock.svelte";
  import { JobStatus, type Job } from "$lib/data/jobs/job";
  import { formatDuration } from "$lib/data/date";

  let { tasks, job }: { tasks: Task[]; job: Job } = $props();

  let now: Date = $state(new Date());

  onMount(() => {
    const timerId = setInterval(() => {
      now = new Date();
    }, 1000);
    return () => {
      clearInterval(timerId);
    };
  });

  const formatTimeElapsed = (job: Job) => {
    if (job.status === JobStatus.Running || job.status === JobStatus.Pending)
      return formatDuration(now.getTime() - (job.queuedAt ?? now).getTime());
    else if (
      job.status === JobStatus.Failed ||
      job.status === JobStatus.Succeeded
    )
      return formatDuration(
        (job.completedAt ?? now).getTime() - (job.queuedAt ?? now).getTime()
      );

    return "";
  };

  const formatTimeRemaining = (job: Job) => {
    if (
      job.status !== JobStatus.Running ||
      job.queuedAt === undefined ||
      job.frameRenderedCount === undefined
    )
      return "";

    const timeElapsed = now.getTime() - job.queuedAt.getTime();
    const estimatedTimeRemaining =
      (timeElapsed * (job.frameCount - job.frameRenderedCount)) /
      job.frameRenderedCount;

    if (estimatedTimeRemaining === Number.NaN) return "";

    return formatDuration(estimatedTimeRemaining);
  };

  let finishedCount = $derived(
    tasks.filter(
      (t) => t.status === TaskStatus.Failed || t.status === TaskStatus.Succeeded
    ).length
  );
  let progress = $derived(finishedCount / tasks.length);
  let timeElapsed = $derived(job ? formatTimeElapsed(job) : "");
  let timeRemaining = $derived(job ? formatTimeRemaining(job) : "");
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

      <p>{Math.floor(progress * 100)}%</p>
    </div>

    <progress
      class="progress progress-success h-1"
      value={finishedCount}
      max={tasks.length}
    ></progress>
  </div>
  <div
    class="collapse-content px-0 overflow-hidden peer-checked:overflow-visible"
  >
    <div class="flex flex-wrap space-x-2 space-y-2 px-2 mt-2">
      <div></div>
      {#each tasks as task}
        <TaskBlock {task} />
      {/each}
    </div>

    <div class="px-4 mt-4 flex flex-col space-y-2">
      {#if timeElapsed}
        <Property label="Time Elapsed" value={timeElapsed}
          ><ClockIcon extraClass="stroke-neutral-content" /></Property
        >
      {/if}
      {#if timeRemaining}
        <Property label="Time Remaining" value={timeRemaining}
          ><QuestionIcon extraClass="stroke-neutral-content" /></Property
        >
      {/if}
    </div>
  </div>
</div>
