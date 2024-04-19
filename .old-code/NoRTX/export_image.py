import PIL as pillow

def export_image(out_array, filename):
    """
    Exports an image from the given numpy array to the specified file.

    Parameters:
    out_array (numpy array): The numpy array (HxWx3) representing the image.
    filename (str): The name of the file to save the image to.
    """
    img = pillow.Image.fromarray(out_array)
    img.save(filename)
    return filename # Why am I doing this?
