import { invoke } from "@tauri-apps/api/core";

enum Commands {
  GET_AWS_CREDENTIALS = "get_aws_credentials",
  UPDATE_AWS_CREDENTIALS = "update_aws_credentials",
}

/** Requests */

interface UpdateAwsCredentialsRequest {
  accessKeyId?: string;
  secretAccessKey?: string;
  region?: string;
}

/** Responses */

interface GetAwsCredentialsResponse {
  accessKeyId?: string;
  secretAccessKey?: string;
  region?: string;
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
