import numpy as np

class RayHandler:
    def __init__(self, resolution: list[int], rpp: int) -> None:
        """
        This class handles rays by storing their directions and positions in a numpy array.

        Args:
            resolution (list[int]): The resolution of the screen in pixels.
            rpp (int): The number of rays to cast per pixel.
        """
        self.rpp = rpp # "Rays Per Pixel"
        self.resolution = resolution

        self.rays = np.zeros(tuple(resolution) + [rpp, 9], dtype=np.float64)  # 3d vector, 3d position, color in single array
        self.pixels = np.zeros(tuple(resolution) + [3], dtype=np.uint8)  # What will be rendered onto the screen

    def step(self, scene) -> None:
        """
        Use this method to parallely process one bounce of rays and store the averages in the `self.pixels` prop.
        """
        # WIP
