use std::rc::Rc;
use bvh::bvh::Bvh;

use crate::ray::Ray;
use crate::camera::Camera;
use crate::hitinfo::HitInfo;
use crate::materials::base::Material;
use crate::geometries::base::Geometry;
use crate::scene::{Scene, SceneObject};
use crate::vectors::{Color, PixelCoord};

pub struct RenderParameters {
    pub max_bounces: u32,
    pub samples: u32
}

pub struct Tracer<'a> {
    pub scene: &'a Scene,
    pub camera: &'a Camera,
    pub bvh: &'a Bvh<f32, 3>
}

impl Tracer<'_> {
    fn get_current_ray(ray: &Ray, hit_info: &Vec<HitInfo>) -> Ray {
        if hit_info.len() > 0 {
            return hit_info[hit_info.len() - 1].bounced.clone()
        } else {
            return ray.clone()
        }
    }

    fn process_bounce(&self, ray: Ray) -> Option<HitInfo> {
        let current_ray = Rc::new(ray);
        let intersections: Vec<&Box<SceneObject>> = self.scene.intersect(self.bvh, Rc::clone(&current_ray));

        if intersections.len() == 0 {
            return None;
        }

        let nearest: &SceneObject = intersections[0].as_ref();
        let geometry: &Box<dyn Geometry + Send + Sync> = &nearest.geometry;
        let material: &Box<dyn Material + Send + Sync> = &nearest.material;

        let (hit_point, normal) = geometry.intersect(&current_ray);
        if !f32::is_nan(hit_point.x) {
            let previous = current_ray.clone();
            let new = material.bounce(&current_ray, hit_point, normal);

            return Some(HitInfo {
                incoming: *previous,
                hit_point,
                normal,
                bounced: new,
                object: Box::new(nearest),
            });
        } else {
            return None;
        }
    }

    fn sample(&self, pixel: &PixelCoord<u32>, max_bounces: u32) -> Color<f32> {
        let mut color: Color<f32> = Color::new(0.0, 0.0, 0.0);
        let mut hit_info: Vec<HitInfo> = vec![];
        let initial_ray: Ray = self.camera.get_ray(*pixel);
        let mut bounces: u32 = 0;

        loop {
            if bounces >= max_bounces {
                break;
            };

            let ray = Tracer::get_current_ray(&initial_ray, &hit_info);
            let hit = self.process_bounce(ray);
            match hit {
                Some(info) => hit_info.push(info),
                None                => {}
            }

            bounces += 1;
        }

        let ray = Tracer::get_current_ray(&initial_ray, &hit_info);
        let norm = ray.direction.normalize();
        let a = 0.5 * (norm.y + 1.0);
        color.x = 255.0 * (1.0 - a) + 0.5 * 255.0 * a;
        color.y = 255.0 * (1.0 - a) + 0.7 * 255.0 * a;
        color.z = 255.0 * (1.0 - a) + 1.0 * 255.0 * a;

        hit_info.reverse();
        for hit in hit_info.iter() {
            let hit_info: &HitInfo = hit.to_owned();
            let object: &SceneObject = hit_info.object.as_ref();
            color = object.material.add_color(&hit.incoming, hit.hit_point, hit.normal, &color);
        }

        return color;
    }

    pub fn get_pixel(&self, pixel: &PixelCoord<u32>, parameters: &RenderParameters) -> Color<f32> {
        let mut color: Color<f32> = Color::new(0.0, 0.0, 0.0);
        let mut samples: u32 = 0;

        loop {
            if samples >= parameters.samples {
                break Default::default();
            }

            let sample = self.sample(pixel, parameters.max_bounces);
            color.x += sample.x;
            color.y += sample.y;
            color.z += sample.z;

            samples += 1;
        }

        color.x /= parameters.samples as f32;
        color.y /= parameters.samples as f32;
        color.z /= parameters.samples as f32;

        return color;
    }
}
