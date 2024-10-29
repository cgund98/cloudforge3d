<script lang="ts">

  import RefreshIcon from "$lib/components/display/icons/RefreshIcon.svelte";
  import SignalIcon from "$lib/components/display/icons/SignalIcon.svelte";
  import Property from "$lib/components/display/Property.svelte";
  import { checkDeploymentHealth, DeploymentStatus } from "$lib/data/settings/commands";
  
  let status: DeploymentStatus = $state(DeploymentStatus.UNKNOWN);

  const fetch = () => {
    checkDeploymentHealth().then(res => {
      status = res.status;
    }).catch(console.error);
  }

  fetch()

  const generateStatusDescription = (status: DeploymentStatus) => {
    if (status === DeploymentStatus.READY)
      return {value: "Ready", tooltip: "Deployment is ready for job submission."}
  
    else if (status === DeploymentStatus.UNREACHABLE)
      return {value: "Unreachable", tooltip: "Unable to reach deployment. Are your AWS credentials correct?"}

    return {value: "Unknown", tooltip: "Unable to determine deployment status."}
  }

  let {value, tooltip} = $derived(generateStatusDescription(status))

</script>
<div class="flex flex-col space-y-0">
    <div class="flex flex-row space-x-4">
      <div class="prose">
        <h3 class="">Queue Status</h3>
      </div>


      <div class="tooltip tooltip-right" data-tip="Refresh Status">
        <button class="btn btn-ghost btn-sm btn-square" onclick={fetch}>
          <RefreshIcon extraClass="size-5" />
        </button>
      </div>
    </div>

    <div class="max-w-xs">
    <Property label="Status" {value} {tooltip}>
        <SignalIcon />
    </Property>
</div>
    </div>
