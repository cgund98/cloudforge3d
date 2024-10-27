import { parseProtoDate } from "$lib/data/date";
import type { Task, TaskStatus } from "$lib/data/tasks/task";
import { invoke } from "@tauri-apps/api/core";

export interface ListTasksRequest {
  jobId: string;
}

export interface ListTasksItem {
  id: string;
  jobId: string;
  frameNumber: number;
  createdAt?: string;
  startedAt?: string;
  queuedAt?: string;
  completedAt?: string;
  status: TaskStatus;
  retryCount: number;
}

export interface ListTasksResponse {
  tasks?: ListTasksItem[];
}

export const listTasks = async (input: ListTasksRequest) => {
  const result = await invoke("list_tasks", { input });

  const raw = result as ListTasksResponse;

  const details: Task[] =
    raw.tasks?.map((t) => ({
      ...t,
      createdAt: t.createdAt ? parseProtoDate(t.createdAt) : undefined,
      queuedAt: t.queuedAt ? parseProtoDate(t.queuedAt) : undefined,
      startedAt: t.startedAt ? parseProtoDate(t.startedAt) : undefined,
      completedAt: t.completedAt ? parseProtoDate(t.completedAt) : undefined,
    })) ?? [];

  return details;
};
