from runner.lib.sqs import read_messages


def main():
    msgs = read_messages(delete=True)

    for msg in msgs:
        # Now, you can access the fields in the Protobuf message
        print(f"Received Message: Task ID = {msg.task_id}, Status = {msg.status}")

    print(f"Received {len(msgs)} messages.")
