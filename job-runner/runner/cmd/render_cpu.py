import os

from runner.lib.blender import render_cycles_cpu_frame
from runner.lib.ocio import apply_and_persist_transform


def main():
    """Entrypoint method"""

    # Render
    output_path = render_cycles_cpu_frame()
    print(f"Rendered file to {output_path}.")

    # Transform
    transform_path = os.path.splitext(output_path)[0] + ".transformed.webp"
    apply_and_persist_transform(output_path, transform_path)
    print(f"Applied ODT and saved to {transform_path}.")


if __name__ == "__main__":
    main()
