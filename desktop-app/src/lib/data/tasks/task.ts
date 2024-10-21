import type { JobStatus } from "../jobState";

export interface Task {
  status: JobStatus;
  frameNumber: number;
  queuedAt: Date;
  startedAt?: Date;
  completedAt?: Date;
}
