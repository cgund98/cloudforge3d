export enum EventName {
  FILE_UPLOAD_PROGRESS = "job-file-upload-progress",
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
