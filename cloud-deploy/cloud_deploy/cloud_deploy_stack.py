from aws_cdk import Duration, Stack, aws_sqs as sqs, aws_s3 as s3

from constructs import Construct


class CloudDeployStack(Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        # The code that defines your stack goes here
        s3.Bucket(self, "blob-store", bucket_name="cf3d-blob-store")

        # Task updates SQS queue
        sqs.Queue(
            self,
            "task-updates",
            queue_name="cf3d-task-updates.fifo",
            fifo=True,
            visibility_timeout=Duration.seconds(30),
        )
