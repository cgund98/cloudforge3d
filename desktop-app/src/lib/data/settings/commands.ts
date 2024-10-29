import { invoke } from "@tauri-apps/api/core";

enum Commands {
  GET_AWS_CREDENTIALS = "get_aws_credentials",
  UPDATE_AWS_CREDENTIALS = "update_aws_credentials",
  CHECK_DEPLOYMENT_HEALTH = "check_deployment_health",
}

export enum DeploymentStatus {
  READY = "ready",
  UNREACHABLE = "unreachable",
  UNKNOWN = "unknown",
}

/** Requests */

interface UpdateAwsCredentialsRequest {
  accessKeyId?: string;
  secretAccessKey?: string;
  region?: string;
}

/** Responses */

export interface GetAwsCredentialsResponse {
  accessKeyId?: string;
  secretAccessKey?: string;
  region?: string;
}

export interface CheckDeploymentHealthResponse {
  status: DeploymentStatus;
}

/** Commands */

export const getAwsCredentials = async () => {
  const result = await invoke(Commands.GET_AWS_CREDENTIALS);

  return result as GetAwsCredentialsResponse;
};

export const updateAwsCredentials = async (
  args: UpdateAwsCredentialsRequest
) => {
  const { accessKeyId = null, secretAccessKey = null, region = null } = args;

  const invokeArgs: Record<string, unknown> = {
    input: {
      accessKeyId,
      secretAccessKey,
      region,
    },
  };

  await invoke(Commands.UPDATE_AWS_CREDENTIALS, invokeArgs);
};

export const checkDeploymentHealth = async () => {
  const result: CheckDeploymentHealthResponse = await invoke(
    Commands.CHECK_DEPLOYMENT_HEALTH
  );

  return result;
};
