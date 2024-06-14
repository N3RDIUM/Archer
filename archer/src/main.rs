use image::{ImageBuffer, RgbImage};

// Constants
const RESOLUTION: [u32; 2] = [10, 10];

fn main() {
    // Make the image
    let mut image: RgbImage = ImageBuffer::new(RESOLUTION[0], RESOLUTION[1]);

    // Modify it
    *image.get_pixel_mut(5, 5) = image::Rgb([255; 3]);

    // Write the image!
    image.save("output.png").unwrap();
}
