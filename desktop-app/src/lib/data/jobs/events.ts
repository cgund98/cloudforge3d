export enum EventName {
  FILE_UPLOAD_PROGRESS = "job-file-upload-progress",
  JOB_STATUS_UPDATE = "job-status-update",
}

export interface FileUploadProgressEvent {
  jobId: string;
  fileName: string;
  currentChunk: number;
  totalChunks: number;
  hasError: boolean;
  isDone: boolean;
  description?: string;
}

export interface JobStatusUpdateEvent {
  jobId: string;
}
