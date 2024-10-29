from aws_cdk import (
    Duration,
    Stack,
    aws_sqs as sqs,
    aws_s3 as s3,
    aws_iam as iam,
    aws_ec2 as ec2,
    aws_batch as batch,
    aws_ecs as ecs,
    Size,
)

from constructs import Construct


class CloudDeployStack(Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        # The code that defines your stack goes here
        bucket = s3.Bucket(self, "blob-store", bucket_name="cf3d-blob-store")

        # Task updates SQS queue
        updates_queue = sqs.Queue(
            self,
            "task-updates",
            queue_name="cf3d-task-updates.fifo",
            fifo=True,
            visibility_timeout=Duration.seconds(10),
            retention_period=Duration.days(14),
        )

        vpc = ec2.Vpc(
            self,
            "Vpc",
            vpc_name="cf3d-vpc",
            max_azs=2,
            nat_gateways=0,  # No NAT Gateway to save costs
            subnet_configuration=[
                ec2.SubnetConfiguration(
                    name="Public",
                    subnet_type=ec2.SubnetType.PUBLIC,
                    cidr_mask=24,
                ),
            ],
        )

        batch_service_role = iam.Role(
            self,
            "BatchServiceRole",
            assumed_by=iam.ServicePrincipal("batch.amazonaws.com"),
            managed_policies=[
                iam.ManagedPolicy.from_aws_managed_policy_name(
                    "service-role/AWSBatchServiceRole"
                ),
            ],
        )

        launch_template = ec2.LaunchTemplate(
            self,
            "BatchLaunchTemplate",
            block_devices=[
                ec2.BlockDevice(
                    device_name="/dev/xvda",
                    volume=ec2.BlockDeviceVolume.ebs(
                        volume_type=ec2.EbsDeviceVolumeType.GP2, volume_size=250
                    ),
                )
            ],
        )

        compute_environment = batch.ManagedEc2EcsComputeEnvironment(
            self,
            "SpotComputeEnvironment",
            compute_environment_name="cf3d-cpu-render-env",
            vpc=vpc,
            allocation_strategy=batch.AllocationStrategy.SPOT_CAPACITY_OPTIMIZED,
            instance_classes=[ec2.InstanceClass.M6I, ec2.InstanceClass.M5],
            minv_cpus=0,
            maxv_cpus=96,
            spot=True,
            service_role=batch_service_role,
            launch_template=launch_template,
        )

        batch.JobQueue(
            self,
            "JobQueue",
            job_queue_name="cf3d-job-queue",
            compute_environments=[
                batch.OrderedComputeEnvironment(
                    compute_environment=compute_environment, order=1
                )
            ],
            priority=1,
        )

        # Create job role
        job_role = iam.Role(
            self, "JobRole", assumed_by=iam.ServicePrincipal("ecs-tasks.amazonaws.com")
        )
        updates_queue.grant_send_messages(job_role)
        bucket.grant_read(job_role)
        bucket.grant_write(job_role)

        region = Stack.of(self).region
        account = Stack.of(self).account
        batch.EcsJobDefinition(
            self,
            "MyJobDefinition",
            job_definition_name="cf3d-cpu-job-definition",
            timeout=Duration.hours(12),
            propagate_tags=True,
            container=batch.EcsEc2ContainerDefinition(
                self,
                "CpuRenderContainerDef",
                job_role=job_role,
                image=ecs.ContainerImage.from_registry(
                    "cgundlach13/cloudforge3d-blender-cpu-render:v0.1.0"
                ),
                memory=Size.mebibytes(1024 * 15),
                cpu=8,
                environment={
                    "AWS_REGION": region,
                },
            ),
        )

        # Create user and access keys for the desktop application
        app_user = iam.User(self, "AppUser", user_name="cf3d-app-user")

        app_group = iam.Group(self, "AppGroup")
        app_group.add_user(app_user)

        app_group.add_to_policy(
            iam.PolicyStatement(
                actions=[
                    "s3:AbortMultipartUpload",
                    "s3:ListBucketMultipartUploads",
                    "s3:*Object",
                    "s3:ListBucket",
                ],
                resources=[bucket.bucket_arn, bucket.bucket_arn + "*"],
            )
        )

        app_group.add_to_policy(
            iam.PolicyStatement(
                actions=["batch:SubmitJob", "batch:CancelJob"],
                resources=[
                    f"arn:aws:batch:{region}:{account}:job-definition/cf3d*",
                    f"arn:aws:batch:{region}:{account}:job-queue/cf3d*",
                    f"arn:aws:batch:{region}:{account}:job/*",
                ],
            )
        )

        # Optionally, allow the role to list job queues if needed
        app_group.add_to_policy(
            iam.PolicyStatement(
                actions=["batch:Describe*", "batch:List*"], resources=["*"]
            )
        )
        updates_queue.grant_consume_messages(app_group)
