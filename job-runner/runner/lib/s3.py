"""
This module includes logic for copying to a from the S3 bucket where render assets are stored.
"""

import os

import boto3

## S3 Client

client = boto3.client("s3")

## Constants

BUCKET_NAME = "cf3d-blob-store"

## Functions


def get_job_input_key(job_id: str, subpath: str) -> str:
    """
    Find the path of an input object in the S3 bucket for a given job.
    """
    return f"jobs/{job_id}/inputs/{subpath}"


def get_job_output_key(job_id: str, subpath: str) -> str:
    """
    Find the path of an output object in the S3 bucket for a given job.
    """
    return f"jobs/{job_id}/outputs/{subpath}"


def copy_file_from_bucket(source_key: str, dest_path: str):
    """
    Copy an object from the S3 bucket to a local path.
    """
    client.download_file(BUCKET_NAME, source_key, dest_path)


def copy_folder_from_bucket(source_key: str, dest_path: str):
    """
    Copy an entire folder from the S3 bucket to a local path.
    """

    # Ensure local path exists
    if not os.path.exists(dest_path):
        os.makedirs(dest_path)

    # List all files in the S3 folder
    response = client.list_objects_v2(Bucket=BUCKET_NAME, Prefix=source_key)

    # Check if the folder contains any files
    if "Contents" in response:
        for obj in response["Contents"]:
            if obj["Size"] == 0:
                continue

            # Get the S3 object key (file path)
            s3_key = obj["Key"]

            # Remove the folder prefix from the key to create the local file path
            local_file_path = os.path.join(
                dest_path, os.path.relpath(s3_key, source_key)
            )

            # Ensure the directory exists locally for the file
            os.makedirs(os.path.dirname(local_file_path), exist_ok=True)

            # Download the file
            print(f"Downloading {s3_key} to {local_file_path}")
            client.download_file(BUCKET_NAME, s3_key, local_file_path)


def copy_file_to_bucket(source_path: str, dest_key: str):
    """
    Copy an object from a local path to the S3 Bucket.
    """
    client.upload_file(source_path, BUCKET_NAME, dest_key)
