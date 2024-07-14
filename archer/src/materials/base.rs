use crate::ray::Ray;
use crate::vectors::Vec3;

pub trait Material {
    fn bounce(&self, incoming: Ray, hit_point: Vec3, normal: Vec3) -> Ray;
    fn add_color(&self, incoming: Ray, hit_point: Vec3, normal: Vec3) -> [u8; 3]; // TODO: Account
                                                                                  // for the
                                                                                  // previous
                                                                                  // color!
}

pub struct BaseMaterial {}
impl Material for BaseMaterial {
    fn bounce(&self, incoming: Ray, hit_point: Vec3, normal: Vec3) -> Ray {
        let _ = (incoming, hit_point, normal);
        return Ray {
            origin: Vec3::new_empty(),
            direction: Vec3::new_empty()
        }
    }

    fn add_color(&self, incoming: Ray, hit_point: Vec3, normal: Vec3) -> [u8; 3] {
        let _ = (incoming, hit_point, normal);
        return [0, 0, 0];
    }
}

