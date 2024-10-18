use bvh::{
    aabb::{Aabb, Bounded},
    bounding_hierarchy::{BHShape, BoundingHierarchy},
    bvh::Bvh,
};
use std::rc::Rc;

use crate::geometries::base::Geometry;
use crate::materials::base::Material;
use crate::ray::Ray;

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
        bvh.nearest_traverse_iterator(&bvh_ray, &self.objects)
            .collect()
    }

    pub fn add(&mut self, object: SceneObject) {
        self.objects.push(Box::new(object));
    }
}
