import numpy as np

class RayHandler:
    def __init__(self, resolution, rpp) -> None:
        """
        This class handles rays by storing their directions and positions in a numpy array.
        """
        self.rpp = rpp # "Rays Per Pixel"
        self.resolution = resolution
        
        self.rays = np.zeros(resolution + [rpp, 6]) # 3d vector, 3d position, color in single array
        self.pixels = np.zeros(resolution) # What will be rendered onto the screen
