<script lang="ts">
  import Document from "$lib/components/display/icons/Document.svelte";
  import DocumentText from "$lib/components/display/icons/DocumentText.svelte";
  import FilmIcon from "$lib/components/display/icons/FilmIcon.svelte";
  import RectangleStack from "$lib/components/display/icons/RectangleStack.svelte";

  import Property from "$lib/components/display/Property.svelte";
  import type {JobDetails} from "$lib/data/jobs/commands/getJob";

  export let job: JobDetails;
</script>

<div class="collapse collapse-arrow  border-base-300 bg-base-200 border">
  <input type="checkbox" class="peer" checked />
  <div class="collapse-title">
    <p class="text-md font-medium">Properties</p>
  </div>
  <div class="collapse-content">
    <div class="grid lg:grid-cols-2 grid-cols-1 gap-2">
      <Property label="File Name" value={job.fileName}
        ><Document/></Property
      >
      <Property label="File Size" value={`${job.fileSizeMb ?? "<1"} MB`}
        ><DocumentText/></Property
      >
      <Property label="Frame Count" value={job.frameCount.toString()}
        ><RectangleStack/></Property
      >
      <Property label="Frame Rate" value={`${job.frameRate} fps`}
        ><FilmIcon/></Property
      >
    </div>
  </div>
</div>
