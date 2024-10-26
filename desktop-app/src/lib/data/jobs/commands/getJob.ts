import type { JobStatus } from "$lib/data/jobState";
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

  console.log(result);

  return result as GetJobResponse;
};
