import io
import subprocess


def render_cycles_cpu_frame() -> str:
    """
    Render a single frame with blender.
    """

    frame_number = 5
    output_base = "/tmp/frame"
    output_template = output_base + "####"
    output_path = output_base + f"{frame_number:04}.exr"

    cmd = [
        "blender",
        "-b",
        "/render/car_render_test.blend",
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
    print("Running command:", cmd)
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)

    logs = []
    for line in io.TextIOWrapper(process.stdout, encoding="utf-8"):
        logs.append(line)
        print(line.rstrip())

    process.stdout.close()
    process.wait()

    return output_path
