"""
This module contains the subprocess for calling the blender command.
"""

import io
import subprocess


def render_cycles_cpu_frame() -> str:
    """
    Render a single frame with blender.

    A wrapper around the blender CLI.
    """
    # TODO - these should be function arguments
    blend_path = "/render/car_render_test.blend"
    frame_number = 5

    # Determine output path. This is where our final frame will be written to.
    output_base = "/tmp/frame"
    output_template = output_base + "####"
    padded_frame = f"{frame_number:04}"
    output_path = output_base + padded_frame + ".exr"

    # Command line arguments
    cmd = [
        "blender",
        "-b",
        blend_path,
        "-o",
        output_template,
        "-F",
        "OPEN_EXR_MULTILAYER",
        "-f",
        str(frame_number),
        "-E",
        "CYCLES",
        "--",
        "--cycles-device",
        "CPU",
    ]

    # Run the blender command.
    # For some reason, blender doesn't log anything to stderr, so we don't capture that output.
    # We do want to capture stdout, though so we can debug a failed job.
    print("Running command:", cmd)
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)

    # Monitor stdout as process runs
    logs = []
    for line in io.TextIOWrapper(process.stdout, encoding="utf-8"):
        logs.append(line)
        print(line.rstrip())

    process.stdout.close()
    process.wait()

    # Write logs to a file so it can be used to debug the job
    logs_path = output_base + padded_frame + ".stdout.txt"
    with open(logs_path, "w") as out:
        out.writelines(logs)

    return output_path, logs_path
