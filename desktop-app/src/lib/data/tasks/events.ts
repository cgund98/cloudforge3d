import type { TaskStatus } from "./task";

export enum EventName {
  TASK_STATUS_UPDATE = "task-status-update",
}

export interface TaskStatusUpdateEvent {
  task_id: string;
  status: TaskStatus;
}
