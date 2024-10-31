import { invoke } from "@tauri-apps/api/core";

export interface UpdateJobRequest {
  jobId: string;
  downloadPath?: string;
}

/**
 * Update a job.
 */
export const updateJob = async (input: UpdateJobRequest) => {
  await invoke("update_job", { input });
};
