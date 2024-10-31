import os

import grpc

import bpy

from cloudforge3d.client import get_client
from cloudforge3d.client.cf3d.v1.jobs_pb2 import CreateJobRequest, CreateJobResponse

class CreateJobOperator(bpy.types.Operator):
    bl_idname = "cf3d.create_job"
    bl_label = "Submit Job"

    def execute(self, context):
        """Create a job"""
        props = context.scene.cf3d

        # Parse properties
        scene = context.scene
        file_path: str = bpy.data.filepath
        frame_start = scene.frame_start
        frame_count = scene.frame_end - frame_start + 1
        frame_rate = scene.render.fps
        name = os.path.basename(file_path)
        download_path = bpy.path.abspath(context.scene.render.filepath)
        ocio_path = os.environ.get("OCIO")
        memory_gb = int(props.memory_gb)

        if props.render_mode == "FRAME":
            frame_start = scene.frame_current
            frame_count = 1

        request = CreateJobRequest(
            file_path=file_path,
            frame_start=frame_start,
            frame_count=frame_count,
            frame_rate=frame_rate,
            download_path=download_path,
            name=name,
            ocio_config_path=ocio_path,
            memory_mib=memory_gb*1024,
        )

        # Pack assets
        bpy.ops.file.pack_all()
        bpy.ops.wm.save_mainfile()

        # Send request
        with grpc.insecure_channel('127.0.0.1:52531') as channel:
            client = get_client(channel)
            response: CreateJobResponse = client.CreateJob(request)
            print(f"Created job with (job_id={response.job_id}).")

        return {"FINISHED"}