use crate::geometries::base::Geometry;
use crate::ray::Ray;
use crate::vectors::dot;
use crate::vectors::Vec3;

pub struct Sphere {
    pub radius: f64,
    pub position: Vec3,
}

impl Geometry for Sphere {
    fn intersect(&self, incoming: Ray) -> (Vec3, Vec3) {
        // Origin
        let oc: Vec3 = self.position - incoming.origin;

        // Coefficients and discriminant
        let a: f64 = dot(incoming.direction, incoming.direction);
        let b: f64 = -2.0 * dot(incoming.direction, oc);
        let c: f64 = dot(oc, oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;

        let ret = Vec3::fill(f64::NAN);
        if discriminant >= 0.0 {
            let t1 = -b - discriminant.sqrt() / (2.0 * a); // Near
            let _t2 = -b + discriminant.sqrt() / (2.0 * a); // Far

            // TODO: Calculate the normal
            let normal = Vec3::fill(f64::NAN);

            return (incoming.position_at(t1), normal);
        }

        return (ret, Vec3::fill(f64::NAN));
    }
}
