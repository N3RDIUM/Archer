use crate::geometries::base::{BaseGeometry, Geometry};
use crate::materials::base::{BaseMaterial, Material};
use crate::ray::Ray;
use crate::vectors::Normal;
use nalgebra::Point3;

pub struct HitInfo<'a> {
    pub incoming: Ray,
    pub hit_point: Point3<f32>,
    pub normal: Normal<f32>,
    pub bounced: Ray,
    pub geometry: Box<&'a (dyn Geometry + Send + Sync)>,
    pub material: Box<&'a (dyn Material + Send + Sync)>,
}

impl HitInfo<'_> {
    pub fn new() -> HitInfo<'static> {
        return HitInfo {
            incoming: Ray::new_empty(),
            hit_point: Point3::new(f32::NAN, f32::NAN, f32::NAN),
            normal: Normal::new(f32::NAN, f32::NAN, f32::NAN),
            bounced: Ray::new_empty(),
            geometry: Box::new(&BaseGeometry {}),
            material: Box::new(&BaseMaterial {}),
        };
    }
}
