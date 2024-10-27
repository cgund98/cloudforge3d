import { parseProtoDate } from "$lib/data/date";
import type { Job, JobStatus } from "../job";
import { invoke } from "@tauri-apps/api/core";

export interface GetJobRequest {
  jobId: string;
}

export interface JobDetails {
  id: string;
  name: string;
  status: JobStatus;
  createdAt?: string;
  queuedAt?: string;
  completedAt?: string;
  fileName: string;
  fileSizeMb: number;
  frameCount: number;
  frameRate: number;
  frameStart: number;
  frameRenderedCount: number;
  downloadPath: string;
  hasPreview: boolean;
}

export interface GetJobResponse {
  job?: JobDetails;
}

export const getJob = async (input: GetJobRequest) => {
  const result = await invoke("get_job", { input });

  const raw = result as GetJobResponse;

  const parsed: Job | undefined = raw.job
    ? {
        ...raw.job,
        createdAt: raw.job.createdAt
          ? parseProtoDate(raw.job.createdAt)
          : undefined,
        queuedAt: raw.job.queuedAt
          ? parseProtoDate(raw.job.queuedAt)
          : undefined,
        completedAt: raw.job.completedAt
          ? parseProtoDate(raw.job.completedAt)
          : undefined,
      }
    : undefined;

  return parsed;
};
