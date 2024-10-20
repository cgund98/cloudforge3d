"""
Entrypoint script for a blender CPU render job.
"""

import os

from runner.lib.blender import render_cycles_cpu_frame
from runner.lib.ocio import apply_and_persist_transform


def main():
    """Entrypoint method"""

    # TODO - parse job and frame from environment

    # TODO - retrieve .blend and OCIO files from S3

    # Render
    output_path, stdout_path = render_cycles_cpu_frame()
    print(f"Rendered file to {output_path}.")
    print(f"Blender logs stored at {stdout_path}.")

    # Apply OCIO transforms
    transform_path = os.path.splitext(output_path)[0] + ".transformed.webp"
    apply_and_persist_transform(output_path, transform_path)
    print(f"Applied ODT and saved to {transform_path}.")

    # TODO - persist results to S3


if __name__ == "__main__":
    main()
