import { JobStatus } from "./job";
import { TaskStatus } from "../tasks/task";

export const mapJobStatusToColor = (status: JobStatus): string => {
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
      return "error";
  }
};

export const mapTaskStatusToColor = (status: TaskStatus): string => {
  switch (status) {
    case TaskStatus.Pending:
      return "info";
    case TaskStatus.Running:
      return "warning";
    case TaskStatus.Succeeded:
      return "success";
    case TaskStatus.Failed:
    case TaskStatus.Canceled:
      return "error";
    default:
      return "error";
  }
};
