<script lang="ts">

    export let label: string;
    export let value: string;
</script>
<div
        class="flex justify-between w-full px-2 py-2 mt-2 border-[1px] border-neutral rounded"
      >
        <div class="flex space-x-3 text-neutral-content">
          <slot />
          <p>{label}</p>
        </div>

        <div>
          <p class="font-semibold">{value}</p>
        </div>
      </div>