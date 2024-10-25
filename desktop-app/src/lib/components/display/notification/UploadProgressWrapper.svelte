<script lang="ts">
  import {
    EventName,
    type FileUploadProgressEvent,
  } from "$lib/data/jobs/events";
  import { listen } from "@tauri-apps/api/event";
  import { getContext, onDestroy } from "svelte";
  import AlertSuccessIcon from "../icons/alert/AlertSuccessIcon.svelte";
  import { scale } from "svelte/transition";
  import { ContextKeys } from "$lib/state";
  import type { Writable } from "svelte/store";
  import type { AlertItem } from "$lib/state/alerts";
  import { genError } from "$lib/state/alerts/factory";

  let curEvent: FileUploadProgressEvent | null = null;
  let lastUpdated = new Date();

  // How long will the element remain visible for once it hasn't been updated.
  const LIFESPAN_MS = 3000;

  let alerts = getContext(ContextKeys.ALERTS) as Writable<AlertItem[]>;

  // Start listening for progress events
  let unMountFn = () => {};
  const init = async () => {
    const unlisten = await listen<string>(
      EventName.FILE_UPLOAD_PROGRESS,
      (e) => {
        curEvent = JSON.parse(e.payload) as FileUploadProgressEvent;
        lastUpdated = new Date();

        if (curEvent.hasError) {
          const newAlert = genError(
            "Encountered error when uploading a file. Are your AWS Credentials correct?"
          );
          alerts.update((cur) => [...(cur ? cur : []), newAlert]);
          curEvent = null;
          return;
        }

        setTimeout(() => {
          const timeSinceUpdate = Date.now() - lastUpdated.getTime();

          if (timeSinceUpdate > LIFESPAN_MS - 1) curEvent = null;
        }, LIFESPAN_MS);
      }
    );
    unMountFn = unlisten;
  };

  init();

  onDestroy(() => {
    unMountFn();
  });

  $: progress = (curEvent?.currentChunk ?? 0) / (curEvent?.totalChunks ?? 1);
  $: bgColor = curEvent?.isDone ? "success" : "info";
</script>

{#if curEvent !== null}
  <div
    class="flex justify-between bg-{bgColor} rounded-2xl text-neutral py-4 px-4"
    transition:scale
  >
    <div class="flex space-x-3">
      {#if curEvent?.isDone}
        <AlertSuccessIcon />
      {:else}
        <span class="loading loading-spinner loading-md"></span>
      {/if}
      <p>Upload in progress...</p>
    </div>

    <p>{curEvent?.fileName} <span>({Math.floor(progress * 100)}%)</span></p>
  </div>
{/if}
