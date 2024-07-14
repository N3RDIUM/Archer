use crate::geometries::base::Geometry;
use crate::vectors::Vec3;
use crate::vectors::dot;
use crate::ray::Ray;

pub struct Sphere {
    pub radius: f32,
    pub position: Vec3
}

impl Geometry for Sphere {
    fn intersect(&self, incoming: Ray) -> (Vec3, Vec3) {
        // Origin
        let oc: Vec3 = self.position - incoming.origin;
        
        // Coefficients and discriminant
        let a: f32 = dot(incoming.direction, incoming.direction);
        let b: f32 = -2.0 * dot(incoming.direction, oc);
        let c: f32 = dot(oc, oc) - self.radius * self.radius;
        let discriminant: f32 = b*b - 4.0*a*c;

        let ret = Vec3::fill(f32::NAN);
        if discriminant >= 0.0 {
            // Solutions
            let t1 = -b - discriminant.sqrt() / (2.0 * a); // Near
            // let t2 = -b + discriminant.sqrt() / (2.0 * a); // Far
            // For d==0, near = far.

            // TODO: Calculate the normal
            let normal = Vec3::fill(f32::NAN);

            return (incoming.position_at(t1), normal); 
        }

        return (ret, Vec3::fill(f32::NAN));
    }
}

