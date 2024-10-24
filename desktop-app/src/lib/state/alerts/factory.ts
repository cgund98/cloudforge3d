import type { AlertItem } from ".";
import { v4 } from "uuid";

export const genError = (msg: string) => {
  const newAlert: AlertItem = {
    id: v4().toString(),
    severity: "error",
    msg,
  };

  return newAlert;
};

export const genSuccess = (msg: string) => {
  const newAlert: AlertItem = {
    id: v4().toString(),
    severity: "success",
    msg,
  };

  return newAlert;
};

export const genInfo = (msg: string) => {
  const newAlert: AlertItem = {
    id: v4().toString(),
    severity: "info",
    msg,
  };

  return newAlert;
};

export const genWarning = (msg: string) => {
  const newAlert: AlertItem = {
    id: v4().toString(),
    severity: "warning",
    msg,
  };

  return newAlert;
};
