export enum JobStatus {
  Pending = "pending",
  Running = "running",
  Succeeded = "succeeded",
  Failed = "failed",
  Canceled = "canceled",
}

export const isCompleted = (status: JobStatus): boolean =>
  [JobStatus.Succeeded, JobStatus.Failed, JobStatus.Canceled].includes(status);
