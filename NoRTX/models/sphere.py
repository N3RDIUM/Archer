class Sphere:
    def __init__(self, center, radius):
        """
        Sphere primitive class with intersect function
        """
        self.center = center
        self.radius = radius
        
    def intersect(self, point, direction):
        """
        Calculate the intersection from a ray going from the given point and direction
        """            
