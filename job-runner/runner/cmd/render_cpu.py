"""
Entrypoint script for a blender CPU render job.
"""

import os

from runner.lib.blender import render_cycles_cpu_frame
from runner.lib.ocio import apply_and_persist_transform
from runner.lib.s3 import (
    copy_file_from_bucket,
    copy_file_to_bucket,
    copy_folder_from_bucket,
)
from runner.lib.spec.cf3d.v1.tasks_pb2 import TaskStatusUpdate, TaskStatus
from runner.lib.sqs import publish_status_update


def parse_args():
    """
    Parse arguments from environment variables.
    """

    # Job ID
    job_id = os.environ.get("CF3D_JOB_ID")
    if job_id is None:
        raise RuntimeError("Unable to parse job id from environment.")

    # Task ID
    task_id = os.environ.get("CF3D_TASK_ID")
    if task_id is None:
        raise RuntimeError("Unable to parse task id from environment")

    # Frame number
    frame_number = os.environ.get("CF3D_FRAME_NUMBER")
    if frame_number is None:
        raise RuntimeError("Unable to parse frame number from environment")
    try:
        frame_number_int = int(frame_number)
    except ValueError:
        raise RuntimeError("Unable to parse frame number to an integer.")

    return job_id, task_id, frame_number_int


def write_logs(logs: list[str], job_id: str, frame_number: int):
    """
    Publish logs to S3 for debugging.
    """
    print("Writing logs to S3...")
    logs_path = "/tmp/logs.txt"
    with open(logs_path, "w") as out:
        out.writelines(logs)

    dest_path = f"jobs/{job_id}/outputs/frame{frame_number:04}.logs.txt"
    copy_file_to_bucket(logs_path, dest_path)


def render_frame(job_id: str, task_id: str, frame_number: int) -> bool:
    """
    Render a single frame
    """

    # Retrieve .blend and OCIO files from S3
    job_subpath = f"jobs/{job_id}"

    ocio_base_path = "/tmp/ocio"
    print("Downloading OCIO configs...")
    copy_folder_from_bucket(job_subpath + "/inputs/ocio/", ocio_base_path)
    ocio_config_path = "/tmp/ocio/config.ocio"

    blend_path = "/tmp/render.blend"
    print("Downloading blend file...")
    copy_file_from_bucket(job_subpath + "/inputs/render.blend", blend_path)

    # Set OCIO environment variable
    os.environ["OCIO"] = ocio_config_path

    # Render
    output_path, logs = render_cycles_cpu_frame(blend_path, frame_number)

    # Exit if render failed
    if output_path is None:
        print("Unable to render frame.")
        write_logs(logs, job_id, frame_number)
        raise RuntimeError("Blender did not return successful status code.")

    # Apply OCIO transforms and generate preview image
    print("Generating preview image...")
    transform_path: str | None = os.path.splitext(output_path)[0] + ".preview.webp"
    try:
        apply_and_persist_transform(output_path, transform_path, ocio_config_path)
        print(f"Applied ODT and saved to {transform_path}.")
    except Exception as ex:
        msg = f"Unable to generate preview: {ex}"
        print(msg)
        logs.append(msg)
        transform_path = None

    # Persist results to S3
    print("Uploading files to s3...")
    write_logs(logs, job_id, frame_number)

    frame_base = job_subpath + f"/outputs/frame{frame_number:04}"
    copy_file_to_bucket(output_path, frame_base + ".exr")

    if transform_path is not None:
        copy_file_to_bucket(transform_path, frame_base + ".preview.webp")


def main():
    """Entrypoint method"""

    # Parse arguments from environment
    job_id, task_id, frame_number = parse_args()

    # Publish job started event
    running_event = TaskStatusUpdate(
        task_id=task_id, status=TaskStatus.TASK_STATUS_RUNNING
    )
    publish_status_update(event=running_event, job_id=job_id)

    try:
        render_frame(job_id, task_id, frame_number)
        succeed_event = TaskStatusUpdate(
            task_id=task_id, status=TaskStatus.TASK_STATUS_SUCCEEDED
        )
        publish_status_update(event=succeed_event, job_id=job_id)
    except Exception as ex:
        failed_event = TaskStatusUpdate(
            task_id=task_id, status=TaskStatus.TASK_STATUS_FAILED
        )
        publish_status_update(event=failed_event, job_id=job_id)
        raise ex


if __name__ == "__main__":
    main()
