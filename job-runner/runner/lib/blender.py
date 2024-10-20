import io
import subprocess


def render_cycles_cpu_sequence():
    """
    Render a sequence of frames with blender.
    """


def render_cycles_cpu_frame():
    """
    Render a single frame with blender.
    """

    cmd = [
        "blender",
        "-b",
        "/render/car_render_test.blend",
        "-o",
        "/render/",
        "-F",
        "OPEN_EXR_MULTILAYER",
        "-f",
        "1",
        "-E",
        "CYCLES",
        "--",
        "--cycles-device",
        "CPU",
    ]
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)

    logs = []
    for line in io.TextIOWrapper(process.stdout, encoding="utf-8"):
        logs.append(line)
        print(line.rstrip())

    process.stdout.close()
    process.wait()
