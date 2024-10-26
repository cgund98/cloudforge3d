import type { JobStatus } from "$lib/data/jobState";
import { invoke } from "@tauri-apps/api/core";

export interface ListJobsRequest {
  limit?: number;
  offset?: number;
}

export interface ListJobsItem {
  id: string;
  name: string;
  status: JobStatus;
  createdAt: string;
}

export interface ListJobsResponse {
  jobs: ListJobsItem[];
  total: number;
}

export const listJobs = async (input: ListJobsRequest) => {
  const result = await invoke("list_jobs", { input });

  console.log(result);

  return result as ListJobsResponse;
};
