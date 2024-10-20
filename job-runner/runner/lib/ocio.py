import OpenEXR
import Imath
import numpy as np
import PyOpenColorIO as OCIO
from PIL import Image


def read_exr(filename) -> np.ndarray:
    """
    Read an EXR file from disk and store in a numpy array.
    """
    # Open the EXR file
    exr_file = OpenEXR.InputFile(filename)

    channel_names = exr_file.header()["channels"].keys()

    # Get the header information
    header = exr_file.header()
    dw = header["dataWindow"]
    width = dw.max.x - dw.min.x + 1
    height = dw.max.y - dw.min.y + 1

    # Define the EXR channel format
    FLOAT = Imath.PixelType(Imath.PixelType.FLOAT)

    # Read the Red, Green, and Blue channels.
    # The channels may vary depending on if this is a multilayer EXR
    is_multilayer = "ViewLayer.Combined.R" in channel_names
    if is_multilayer:
        red_channel = exr_file.channel("ViewLayer.Combined.R", FLOAT)
        green_channel = exr_file.channel("ViewLayer.Combined.G", FLOAT)
        blue_channel = exr_file.channel("ViewLayer.Combined.B", FLOAT)
    else:
        red_channel = exr_file.channel("R", FLOAT)
        green_channel = exr_file.channel("G", FLOAT)
        blue_channel = exr_file.channel("B", FLOAT)

    # Convert the byte data to numpy arrays and reshape them
    red = np.frombuffer(red_channel, dtype=np.float32).reshape(height, width)
    green = np.frombuffer(green_channel, dtype=np.float32).reshape(height, width)
    blue = np.frombuffer(blue_channel, dtype=np.float32).reshape(height, width)

    # Stack the channels into a single 3D array (height x width x 3)
    exr_data = np.stack([red, green, blue], axis=-1)

    return exr_data


def apply_display_transform(
    image_data: np.ndarray, ocio_config_path: str | None
) -> np.ndarray:
    """
    Apply a display view transform to an image.
    """
    # Load the OCIO configuration
    if ocio_config_path is None:
        config = OCIO.GetCurrentConfig()
    else:
        config = OCIO.Config.CreateFromFile(ocio_config_path)

    # Create a processor
    display = config.getDefaultDisplay()
    view = config.getDefaultView(display)
    transform = OCIO.DisplayViewTransform()
    transform.setSrc(OCIO.ROLE_SCENE_LINEAR)
    transform.setDisplay(display)
    transform.setView(view)
    processor = config.getProcessor(transform)
    cpu = processor.getDefaultCPUProcessor()

    # Apply the transform to the image data
    # This transformation happens pixel-by-pixel
    transformed_data = np.empty_like(image_data)
    for y in range(image_data.shape[0]):
        for x in range(image_data.shape[1]):
            pixel = image_data[y, x].tolist()
            transformed_data[y, x, :] = cpu.applyRGB(pixel)

    return transformed_data


def save_as_webp(image_data: np.ndarray, output_filename: str) -> None:
    """
    Save the image as a WEBP file with the highest quality.
    """
    # Normalize the pixel values to the 0-255 range for saving as an 8-bit image
    image_data = np.clip(image_data, 0, 1)
    image_data = (image_data * 255).astype(np.uint8)

    # Convert the numpy array to a PIL image
    image = Image.fromarray(image_data)

    # Save the image as a WEBP file
    image.save(output_filename, "WEBP", quality=100, lossless=True)


def apply_and_persist_transform(
    source_path: str, dest_path: str, ocio_config_path: str | None = None
) -> None:
    """
    Load an EXR file, apply a display transform, and persist the result as a WEBP file.
    """

    loaded_data = read_exr(source_path)
    transformed_data = apply_display_transform(loaded_data, ocio_config_path)
    save_as_webp(transformed_data, dest_path)
