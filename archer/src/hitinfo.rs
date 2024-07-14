use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::materials::base::Material;
use crate::materials::base::BaseMaterial;

pub struct HitInfo<'a> {
    pub incoming: Ray,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub bounced: Ray,
    pub material: Box<&'a (dyn Material + Send + Sync)>
}

impl HitInfo<'_> {
    pub fn new() -> HitInfo<'static> {
        return HitInfo {
            incoming: Ray::new_empty(),
            hit_point: Vec3::new_empty(),
            normal: Vec3::new_empty(),
            bounced: Ray::new_empty(),
            material: Box::new(&BaseMaterial{})
        }; 
    }
}

