use std::rc::Rc;
use bvh::bvh::Bvh;
use bvh::aabb::{Aabb, Bounded};
use bvh::bounding_hierarchy::{BHShape, BoundingHierarchy};

use crate::ray::Ray;
use crate::materials::base::Material;
use crate::geometries::base::Geometry;
pub struct SceneObject {
    pub geometry: Box<dyn Geometry + Send + Sync>,
    pub material: Box<dyn Material + Send + Sync>,
    pub node_index: usize,
}

impl Bounded<f64, 3> for SceneObject {
    fn aabb(&self) -> Aabb<f64, 3> {
        self.geometry.aabb()
    }
}

impl BHShape<f64, 3> for SceneObject {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

pub struct Scene {
    pub objects: Vec<Box<SceneObject>>,
}

impl Scene {
    pub fn build_bvh(&mut self) -> Bvh<f64, 3> {
        Bvh::build_par(&mut self.objects)
    }

    pub fn intersect<'a>(
        &'a self,
        bvh: &'a Bvh<f64, 3>,
        ray: Rc<Ray>,
    ) -> Vec<&'a Box<SceneObject>> {
        let bvh_ray = ray.to_bvh_ray();
        return bvh
            .nearest_traverse_iterator(&bvh_ray, &self.objects)
            .collect();
    }

    pub fn add(&mut self, object: SceneObject) {
        self.objects.push(Box::new(object))
    }
}
