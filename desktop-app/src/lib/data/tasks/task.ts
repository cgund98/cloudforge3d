export enum TaskStatus {
  Pending = "pending",
  Running = "running",
  Succeeded = "succeeded",
  Failed = "failed",
}

export interface Task {
  status: TaskStatus;
  frameNumber: number;
  queuedAt: Date;
  startedAt?: Date;
  completedAt?: Date;
}
