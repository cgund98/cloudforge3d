import { invoke } from "@tauri-apps/api/core";

export interface CancelJobRequest {
  jobId: string;
}

/**
 * Cancel a job.
 */
export const cancelJob = async (jobId: string) => {
  const input: CancelJobRequest = { jobId };
  await invoke("cancel_job", { input });
};
