export enum TaskStatus {
  Pending = "pending",
  Running = "running",
  Succeeded = "succeeded",
  Failed = "failed",
}

export interface Task {
  id: string;
  jobId: string;
  frameNumber: number;
  createdAt?: Date;
  startedAt?: Date;
  queuedAt?: Date;
  completedAt?: Date;
  status: TaskStatus;
  retryCount: number;
}
