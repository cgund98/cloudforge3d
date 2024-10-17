<script lang="ts">
  import OpenIcon from "../../icons/OpenIcon.svelte";

  enum JobStatus {
    Pending = "pending",
    Running = "running",
    Succeeded = "succeeded",
    Failed = "failed",
  }

  interface Row {
    name: string;
    status: JobStatus;
    createdAt: Date;
  }

  const rows: Row[] = [
    {
      name: "first job",
      status: JobStatus.Pending,
      createdAt: new Date(),
    },
    {
      name: "second job",
      status: JobStatus.Running,
      createdAt: new Date(Date.now() - 1000 * 60 * 60 * 24),
    },
    {
      name: "third job",
      status: JobStatus.Succeeded,
      createdAt: new Date(Date.now() - 1000 * 60 * 60 * 24 * 2),
    },
    {
      name: "fourth job",
      status: JobStatus.Failed,
      createdAt: new Date(Date.now() - 1000 * 60 * 60 * 24 * 24),
    },
  ];

  // Transformations
  const mapStatusToColor = (status: JobStatus): string => {
    switch (status) {
        case JobStatus.Pending: return "badge-info"
        case JobStatus.Running: return "badge-warning"
        case JobStatus.Succeeded: return "badge-success"
        case JobStatus.Failed: return "badge-error"
    }
  }

  const formatDate = (date: Date): string => {
    const now = new Date()

    const timeOfDay = `${(now.getHours() + 1) % 13}:${String(now.getMinutes()).padStart(2, '0')} ${now.getHours() ? "PM" : "AM" }`

    if (date.toDateString() === now.toDateString()) return `Today, ${timeOfDay}`

    const weekAgo = new Date(Date.now() - 1000 * 60 * 60 * 24 * 7)
    const dayOfWeek = date.toLocaleString('en-us', {  weekday: 'long' })
    if (date.toDateString() !== now.toDateString() && date > weekAgo) return `${dayOfWeek}, ${timeOfDay}`

    const month = date.toLocaleDateString('en-us', { month: 'long'})

    return `${month} ${date.getDate()}, ${timeOfDay}`
  }
</script>

<div class="overflow-x-auto">
  <table class="table table-auto w-full">
    <!-- head -->
    <thead>
      <tr>
        <th>Name</th>
        <th class="text-right">Status</th>
        <th class="text-right">Created At</th>
        <th class="text-right"></th>
      </tr>
    </thead>

    <!-- body -->
    <tbody>
      {#each rows as row, idx}
        <tr>
          <td>{row.name}</td>
          <td class="text-right"><div class="capitalize badge badge-outline {mapStatusToColor(row.status)}">{row.status}</div></td>
          <td class="text-right">{formatDate(row.createdAt)}</td>
          <td class="text-right">
            <button class="btn btn-ghost btn-sm btn-square">
              <OpenIcon />
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
