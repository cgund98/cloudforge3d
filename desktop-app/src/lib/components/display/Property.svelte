<script lang="ts">
  export let label: string;
  export let value: string;
</script>

<div
  class="flex justify-between w-full px-2 py-2 border-[1px] border-neutral rounded"
>
  <div class="flex space-x-3 text-neutral-content">
    <slot />
    <p class="truncate max-w-[100px]">{label}</p>
  </div>

  <div class="tooltip" data-tip={value}>
    <p class="font-semibold truncate max-w-[100px]">{value}</p>
  </div>
</div>
