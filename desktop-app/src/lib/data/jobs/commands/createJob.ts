import { invoke } from "@tauri-apps/api/core";

interface CreateJobRequest {
  filePath: string;
  frameStart: number;
  frameCount: number;
  frameRate: number;
  downloadPath: string;
  name?: string;
  ocioConfigPath?: string;
}

interface CreateJobResponse {
  jobId: string;
}

export const createJob = async () => {
  const input: CreateJobRequest = {
    filePath: "/Users/callum/Downloads/cube.blend",
    frameStart: 1,
    frameCount: 1,
    frameRate: 24,
    downloadPath: "/tmp",
    ocioConfigPath: "/Users/callum/art/config/ocio/PBR_Neutral/config.ocio",
    name: "Fake Job",
  };

  const result = await invoke("create_job", { input });

  return result as CreateJobResponse;
};
