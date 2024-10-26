export enum JobStatus {
  Uploading = "uploading",
  UploadFailed = "upload-failed",
  Pending = "pending",
  Running = "running",
  Succeeded = "succeeded",
  Failed = "failed",
  Canceled = "canceled",
}

export const isCompleted = (status: JobStatus): boolean =>
  [
    JobStatus.Succeeded,
    JobStatus.Failed,
    JobStatus.Canceled,
    JobStatus.UploadFailed,
  ].includes(status);
