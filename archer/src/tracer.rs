use crate::types::{Position, Direction, Color, PixelCoord};
use crate::ray::Ray;
use crate::camera::Camera;
use sdl2::render::Texture;

pub struct Tracer<'a> {
    pub camera: Camera,
    pub texture: &'a Texture<'a>,
}

