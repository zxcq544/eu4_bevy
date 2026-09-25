import json


def build_flags_texture_atlas(flags_dir, output_dir):
    import os
    import sys
    from PIL import Image

    # Get all the flags in the flags directory
    flags = [f for f in os.listdir(flags_dir) if f.endswith(".tga")]
    # Sort the flags by their name
    flags.sort(key=lambda x: x.lower())
    num_flags = len(flags)
    print(f"Found {num_flags} flags in {flags_dir}")
    num_columns = 32
    num_rows = 33

    flag_posiiton_row_column = {}
    for i in range(num_flags):
        flag = flags[i]
        flag_name = flag.split(".")[0]
        flag_posiiton_row_column[flag_name] = {
            "index": i,
            "row": i // num_columns,
            "column": i % num_columns,
        }
    print(f"Flags positions: {flag_posiiton_row_column}")

    # Create the output directory if it doesn't exist
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    # Create the output file
    output_file = os.path.join(output_dir, "flags.tga")

    # Create the output image
    size = 128
    width = num_columns * size
    height = num_rows * size
    output_image = Image.new("RGB", (width, height), (0, 0, 0))

    # Loop through the flags and add them to the output image
    for i, flag in enumerate(flags):
        flag_name = flag.split(".")[0]
        print(f"Adding flag {flag_name}")
        # Get the position of the flag in the output image
        row = flag_posiiton_row_column[flag_name]["row"]
        column = flag_posiiton_row_column[flag_name]["column"]
        x = column * size
        y = row * size
        # Load the flag image
        flag_image = Image.open(os.path.join(flags_dir, flag))
        # Resize the flag image to the size of the output image
        flag_image = flag_image.resize((size, size))
        # Paste the flag image onto the output image
        output_image.paste(flag_image, (x, y))
        

    # Save the output image
    output_image.save(output_file)
    file_info = {}
    file_info["width"] = width
    file_info["height"] = height
    file_info["num_columns"] = num_columns
    file_info["num_rows"] = num_rows
    file_info["flags"] = flag_posiiton_row_column
    json_file = os.path.join(output_dir, "flags.json")
    with open(json_file, "w") as f:
        json.dump(file_info, f, indent=4)

    # print(f"Flags texture atlas saved to {output_file}")


if __name__ == "__main__":
    build_flags_texture_atlas(
        flags_dir="../assets/gfx/flags", output_dir="../assets/gfx/flags/atlas"
    )
