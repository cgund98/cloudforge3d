"""
This module provides functions for publishing task status updates to sqs.
"""

from datetime import datetime
import hashlib
import os
import base64

import boto3
from runner.lib.spec.cf3d.v1.tasks_pb2 import TaskStatusUpdate

## SQS Queue
QUEUE_NAME = "cf3d-task-updates.fifo"
AWS_REGION = os.environ.get("AWS_REGION")

sqs = boto3.resource("sqs", region_name=AWS_REGION)
queue = sqs.get_queue_by_name(QueueName=QUEUE_NAME)


## Functions


def publish_status_update(event: TaskStatusUpdate, job_id: str):
    """
    Publish a task status update event to the SQS queue.
    """

    event_bytes = event.SerializeToString()
    event_base64_byte = base64.b64encode(event_bytes)
    event_base64_str = event_base64_byte.decode("utf-8")

    # Determine deduplication ID
    dedupe_str = bytes(f"{event.task_id} + {datetime.now().isoformat()}", "utf-8")
    dedupe_key = hashlib.sha256(dedupe_str).hexdigest()

    queue.send_message(
        MessageBody=event_base64_str,
        MessageDeduplicationId=dedupe_key,
        MessageGroupId=job_id,
    )


def read_messages(delete=False) -> list[TaskStatusUpdate]:
    """
    This is used to read and print out updates from the SQS queue.

    Use only for debugging.
    """

    messages = queue.receive_messages(
        MaxNumberOfMessages=5,  # Receive up to 5 messages at a time
        WaitTimeSeconds=5,  # Long polling for up to 5 seconds
    )

    result = []
    for msg in messages:
        # Extract the message body (this is base64-encoded Protobuf)
        encoded_body = msg.body

        # Decode the base64-encoded message
        binary_message = base64.b64decode(encoded_body)

        # Deserialize the Protobuf message
        parse_msg = TaskStatusUpdate()
        parse_msg.ParseFromString(binary_message)
        result.append(parse_msg)

        if delete:
            msg.delete()

    return result
