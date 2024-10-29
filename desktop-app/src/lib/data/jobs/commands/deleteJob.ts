import { invoke } from "@tauri-apps/api/core";

export interface DeleteJobRequest {
  jobId: string;
}

/**
 * Delete a job.
 */
export const deleteJob = async (jobId: string) => {
  const input: DeleteJobRequest = { jobId };
  await invoke("delete_job", { input });
};
