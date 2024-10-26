import { JobStatus } from "../jobState";

export const mapStatusToColor = (status: JobStatus): string => {
  switch (status) {
    case JobStatus.Uploading:
    case JobStatus.Pending:
      return "info";
    case JobStatus.Running:
      return "warning";
    case JobStatus.Succeeded:
      return "success";
    case JobStatus.Failed:
    case JobStatus.Canceled:
    case JobStatus.UploadFailed:
      return "error";
    default:
      return "info";
  }
};
