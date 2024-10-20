# Job Runner

This is the container responsible for running render jobs. It consists of a blender installation and a lightweight python wrapper.

### Application Flow

1. Pull .blend file and OCIO configuration (if applicable) from S3.
2. Render frame.
3. Apply OCIO transforms to frame to create a "preview" frame.
4. Persist raw frame, preview frame, and logs to S3.

### Testing Locally

We can use `docker compose` to build and test the container locally.

```bash
# Build the container
docker compose build blender

# Run the container
docker compse run blender
```
