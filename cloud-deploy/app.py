#!/usr/bin/env python3

import aws_cdk as cdk

from cloud_deploy.cloud_deploy_stack import CloudDeployStack


app = cdk.App()
stack = CloudDeployStack(app, "CloudForge3dStack", stack_name="cloudforge3d-stack")

cdk.Tags.of(stack).add("app", "cloudforge3d")

app.synth()
