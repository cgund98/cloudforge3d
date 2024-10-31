<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  export let label: string;
  export let value: string;

  export let handle: (newValue: string) => void;

  const placeholder = "No Path Specified";
</script>

<div
  class="flex justify-between w-full px-2 py-2 border-[1px] border-base-300 rounded items-center"
>
  <div class="flex space-x-3">
    <slot />
    <p class="truncate max-w-[150px]">{label}</p>
  </div>

  <div class="tooltip tooltip-left w-full max-w-[200px]" data-tip={value}>
    <input
      onclick={async () => {
        const newPath = await open({
          directory: true,
          multiple: false,
        });

        if (typeof newPath === "string") handle(newPath);
      }}
      {placeholder}
      readonly
      {value}
      class="input italic input-xs w-full"
    />
  </div>
</div>
