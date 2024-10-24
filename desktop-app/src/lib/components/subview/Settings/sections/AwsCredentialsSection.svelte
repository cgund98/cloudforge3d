<script lang="ts">
  import EditIcon from "$lib/components/display/icons/EditIcon.svelte";
  import {
    getAwsCredentials,
    updateAwsCredentials,
  } from "$lib/data/settings/commands";
  import { ContextKeys } from "$lib/state";
  import type { AlertItem } from "$lib/state/alerts";
  import { genError } from "$lib/state/alerts/factory";
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";


  let alerts = getContext(ContextKeys.ALERTS) as Writable<AlertItem[]>;

  // Form values
  let region = $state("");
  let accessKeyId = $state("");
  let secretAccessKey = $state("");

  // Edit mode
  let editMode: boolean = $state(false);

  const fetchAndPopulateFields = () =>
    getAwsCredentials().then((res) => {
      region = res.region ?? "";
      accessKeyId = res.accessKeyId ?? "";
      secretAccessKey = res.secretAccessKey ?? "";
    });

  const submit = () => {
    updateAwsCredentials({
      region,
      accessKeyId,
      secretAccessKey,
    })
      .then(() => {
        fetchAndPopulateFields().then(() => (editMode = false));
      })
      .catch((err) => {
        console.error(err)
        alerts.update(cur => [...cur, genError("Encountered error when setting AWS credentials.")])
      });
  };

  fetchAndPopulateFields();
</script>

<div class="flex flex-col space-y-0">
  <div class="flex flex-row space-x-4">
    <div class="prose">
      <h3 class="">AWS Credentials</h3>
    </div>

    {#if !editMode}
      <div class="tooltip tooltip-right" data-tip="Edit">
        <button
          on:click={() => {
            fetchAndPopulateFields();
            editMode = true;
          }}
          class="btn btn-ghost btn-sm btn-square"
        >
          <EditIcon extraClass="size-5" />
        </button>
      </div>
    {/if}
  </div>

  <label class="form-control w-full max-w-xs">
    <div class="label">
      <span class="label-text">Region</span>
    </div>
    <input
      type="text"
      placeholder="Not set"
      bind:value={region}
      readonly={!editMode}
      class="input input-bordered w-full max-w-xs"
    />
  </label>

  <label class="form-control w-full max-w-xs">
    <div class="label">
      <span class="label-text">Access Key ID</span>
    </div>
    <input
      type="text"
      placeholder="Not set"
      bind:value={accessKeyId}
      readonly={!editMode}
      class="input input-bordered w-full max-w-xs"
    />
  </label>

  <label class="form-control w-full max-w-xs">
    <div class="label">
      <span class="label-text">Secret Access Key</span>
    </div>
    <input
      type="text"
      placeholder="Not set"
      bind:value={secretAccessKey}
      readonly={!editMode}
      class="input input-bordered w-full max-w-xs"
    />
  </label>

  {#if editMode}
    <div class="flex flex-row space-x-2 pt-4">
      <button class="btn btn-primary" on:click={submit}>Save Changes</button>
      <button
        class="btn btn-neutral"
        on:click={() => {
          fetchAndPopulateFields();
          editMode = false;
        }}>Reset Changes</button
      >
    </div>
  {/if}
</div>
