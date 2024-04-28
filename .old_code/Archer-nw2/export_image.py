# Imports
from PIL import Image

def export_image(array, filename):
    """
    Converts the array into a pillow image
    and then saves it to disk
    """
    image = Image.fromarray(array)
    image.save(filename)
