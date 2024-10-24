/** State types */
type AlertSeverity = "info" | "warning" | "error" | "success";

export interface AlertItem {
  id: string;
  msg: string;
  severity: AlertSeverity;
}