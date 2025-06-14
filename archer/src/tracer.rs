use crate::types::{Position, Direction, Color, PixelCoord};
use crate::ray::Ray;
use crate::camera::Camera;
use sdl2::render::Texture;

pub struct Tracer {
    pub camera: Camera,
}

impl Tracer {
    pub fn new(camera: Camera) -> Tracer {
        Tracer { camera }
    }

    pub fn render(&mut self, texture: &mut Texture) -> Result<(), Box<dyn std::error::Error>> {
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let resolution = self.camera.resolution;

            for x in 0..(resolution.x as usize) {
                for y in 0..(resolution.y as usize) {
                    let offset = y * pitch + x * 3;

                    buffer[offset] = 0;
                    buffer[offset + 1] = 0;
                    buffer[offset + 2] = 0;
                }
            }
        })?;

        Ok(())
    }
}

