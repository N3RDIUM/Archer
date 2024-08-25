use crate::geometries::base::Geometry;
use crate::materials::base::Material;
use crate::ray::Ray;
use bvh::aabb::{Aabb, Bounded};
use bvh::bounding_hierarchy::{BHShape, BoundingHierarchy};
use bvh::bvh::Bvh;
use std::rc::Rc;

pub struct SceneObject {
    pub geometry: Box<dyn Geometry + Send + Sync>,
    pub material: Box<dyn Material + Send + Sync>,
    pub node_index: usize,
}

impl Bounded<f32, 3> for SceneObject {
    fn aabb(&self) -> Aabb<f32, 3> {
        self.geometry.aabb()
    }
}

impl BHShape<f32, 3> for SceneObject {
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
    pub fn build_bvh(&mut self) -> Bvh<f32, 3> {
        Bvh::build_par(&mut self.objects)
    }

    pub fn intersect<'a>(
        &'a self,
        bvh: &'a Bvh<f32, 3>,
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
