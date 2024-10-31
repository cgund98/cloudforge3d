import { invoke } from "@tauri-apps/api/core";

export interface DownloadFramesRequest {
  jobId: string;
}

export interface DownloadFramesResponse {
  downloadedCount?: number;
}

/**
 * Download frames for a job.
 */
export const downloadFrames = async (jobId: string) => {
  const input: DownloadFramesRequest = { jobId };
  const response: DownloadFramesResponse = await invoke("download_frames", { input });

  return response;
};
