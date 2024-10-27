export interface Job {
  id: string;
  name: string;
  status: JobStatus;
  createdAt?: Date;
  queuedAt?: Date;
  completedAt?: Date;
  fileName: string;
  fileSizeMb: number;
  frameCount: number;
  frameRate: number;
  frameStart: number;
  frameRenderedCount: number;
  downloadPath: string;
  hasPreview: boolean;
}

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
