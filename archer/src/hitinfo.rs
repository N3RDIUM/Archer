use nalgebra::Point3;

use crate::ray::Ray;
use crate::scene::SceneObject;
use crate::vectors::Normal;

pub struct HitInfo<'a> {
    pub incoming: Ray,
    pub hit_point: Point3<f32>,
    pub normal: Normal<f32>,
    pub bounced: Ray,
    pub object: Box<&'a SceneObject>,
}