from runner.lib.ocio import apply_and_persist_transform


def main():
    # Example usage
    exr_file = "/home/callum/Downloads/car_test/0001.exr"
    ocio_config = (
        "/home/callum/Downloads/car_test/config.ocio"  # Path to the OCIO config file
    )
    output_file = "/home/callum/Downloads/car_test/0001.transformed.webp"

    apply_and_persist_transform(exr_file, output_file, ocio_config)


if __name__ == "__main__":
    main()
