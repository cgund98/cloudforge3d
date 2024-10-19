<script>
  import AwsCredentialsSection from "./sections/AwsCredentialsSection.svelte";
  import QueueStatusSection from "./sections/QueueStatusSection.svelte";

</script>
<div class="px-4 pt-8 flex flex-col flex-1 space-y-6">
    <div class="prose mb-2">
      <h2 class="mb-0">Settings</h2>
    </div>

    <QueueStatusSection />

    <AwsCredentialsSection />

    </div>
