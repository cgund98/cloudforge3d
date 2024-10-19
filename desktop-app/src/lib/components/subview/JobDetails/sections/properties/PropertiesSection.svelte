<script lang="ts">
  import ClockIcon from "$lib/components/display/icons/ClockIcon.svelte";
  import Document from "$lib/components/display/icons/Document.svelte";
  import DocumentText from "$lib/components/display/icons/DocumentText.svelte";
  import FilmIcon from "$lib/components/display/icons/FilmIcon.svelte";
  import QuestionIcon from "$lib/components/display/icons/QuestionIcon.svelte";
  import RectangleStack from "$lib/components/display/icons/RectangleStack.svelte";

  import Property from "$lib/components/display/Property.svelte";
</script>

<div class="collapse collapse-arrow  border-base-300 bg-base-200 border">
  <input type="checkbox" class="peer" checked />
  <div class="collapse-title">
    <p class="text-md font-medium">Properties</p>
  </div>
  <div class="collapse-content">
    <div class="grid lg:grid-cols-2 grid-cols-1 gap-2">
      <Property label="File Name" value="v_lookdev_2020_09_04.blend"
        ><Document/></Property
      >
      <Property label="File Size" value="5:23:12"
        ><DocumentText/></Property
      >
      <Property label="Frame Count" value="240"
        ><RectangleStack/></Property
      >
      <Property label="Frame Rate" value="24"
        ><FilmIcon/></Property
      >
    </div>
  </div>
</div>
