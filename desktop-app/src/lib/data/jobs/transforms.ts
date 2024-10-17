import { JobStatus } from "../jobState";

export const mapStatusToColor = (status: JobStatus): string => {
  switch (status) {
    case JobStatus.Pending:
      return "info";
    case JobStatus.Running:
      return "warning";
    case JobStatus.Succeeded:
      return "success";
    case JobStatus.Failed:
      return "error";
    case JobStatus.Canceled:
      return "error";
  }
};
