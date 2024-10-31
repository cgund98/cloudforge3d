Job Runner
==========

This container is responsible for running render jobs with a pre-installed Blender environment and a lightweight Python wrapper for job management.

### Limitations

- Currently supports **CPU rendering only**.
- Blender version hard-coded to `4.2`. 

Application Flow
----------------

The rendering process follows these main steps:

1.  Publish an initial status update to **SQS**.
2.  Retrieve the `.blend` file and OCIO configuration (if applicable) from **S3**.
3.  Render the frame.
4.  Apply OCIO transforms to the frame to create a **preview** frame.
5.  Save the raw frame, preview frame, and logs to **S3**.
6.  Publish a final status update to **SQS**.

Development
-----------

### Testing Locally

To test the container locally, use `docker compose` to build and run it.

```bash
# Generate Protobufs
make gen

# Build the container
make build

# Run the container
docker compose run blender
```

### Environment Variables

These environment variables configure the job runner and provide access to AWS resources.

- `CF3D_JOB_ID`: Specifies the unique identifier for the render job.
- `CF3D_TASK_ID`: Identifies the specific task within the job. Each frame gets its own task.
- `CF3D_FRAME_NUMBER`: Sets the frame number to be rendered for this task.
- `AWS_REGION`: The AWS region where the resources (e.g., S3 buckets, SQS queues) are located. This region setting ensures that requests are directed to the correct AWS endpoints.
- `AWS_ACCESS_KEY_ID`: Your AWS access key, required for authentication when accessing AWS resources.
- `AWS_SECRET_ACCESS_KEY`: The AWS secret access key paired with AWS_ACCESS_KEY_ID. This key provides secure access to AWS resources and should be kept private.

> **Note**: For sensitive information like AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY, ensure these variables are stored securely, such as in environment configuration files or secret managers.

Deployment
----------

The container image is published to **DockerHub** for easy deployment.

```bash
# Generate Protobufs (only if the schema changed)
make gen

# Build the container
make build

# Push the container to DockerHub
docker compose push
```