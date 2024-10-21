# Job Runner

This is the container responsible for running render jobs. It consists of a blender installation and a lightweight python wrapper.

### Application Flow

1. Publish initial status update to SQS.
2. Pull .blend file and OCIO configuration (if applicable) from S3.
3. Render frame.
4. Apply OCIO transforms to frame to create a "preview" frame.
5. Persist raw frame, preview frame, and logs to S3.
6. Publish final status update to SQS.

### Testing Locally

We can use `docker compose` to build and test the container locally.

```bash
# Build the container
make build

# Run the container
docker compse run blender
```
