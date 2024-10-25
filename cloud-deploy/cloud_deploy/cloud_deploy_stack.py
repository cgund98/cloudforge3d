from aws_cdk import (
    Duration, 
    Stack, 
    aws_sqs as sqs, 
    aws_s3 as s3, 
    aws_iam as iam, 
    aws_ssm as ssm,
)

from constructs import Construct


class CloudDeployStack(Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        # The code that defines your stack goes here
        bucket = s3.Bucket(self, "blob-store", bucket_name="cf3d-blob-store")

        # Task updates SQS queue
        sqs.Queue(
            self,
            "task-updates",
            queue_name="cf3d-task-updates.fifo",
            fifo=True,
            visibility_timeout=Duration.seconds(30),
        )

        # Create user and access keys for the desktop application
        app_user = iam.User(self, "AppUser", user_name="cf3d-app-user")
        
        app_group = iam.Group(self, "AppGroup")
        app_group.add_user(app_user)

        app_group.add_to_policy(iam.PolicyStatement(
            actions=[
                "s3:AbortMultipartUpload",
                "s3:ListBucketMultipartUploads",
                "s3:PutObject",
                "s3:GetObject"
            ],
            resources=[bucket.bucket_arn, bucket.bucket_arn + "*"]
        ))
