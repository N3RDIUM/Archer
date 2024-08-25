use crate::ray::Ray;
use crate::scene::SceneObject;
use crate::vectors::Normal;
use nalgebra::Point3;

pub struct HitInfo<'a> {
    pub incoming: Ray,
    pub hit_point: Point3<f32>,
    pub normal: Normal<f32>,
    pub bounced: Ray,
    pub object: Box<&'a SceneObject>,
}