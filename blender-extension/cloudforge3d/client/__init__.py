from cloudforge3d.client.cf3d.v1 import jobs_pb2_grpc

def get_client(channel) -> jobs_pb2_grpc.JobsServiceStub:
    return jobs_pb2_grpc.JobsServiceStub(channel)