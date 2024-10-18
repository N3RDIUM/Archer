use bvh::bvh::Bvh;
use core::f64;
use nalgebra::distance;
use rayon::prelude::*;
use std::rc::Rc;

use crate::camera::Camera;
use crate::hitinfo::HitInfo;
use crate::ray::Ray;
use crate::scene::{Scene, SceneObject};
use crate::vectors::{Color, PixelCoord};

pub struct RenderParameters {
    pub max_bounces: u32,
    pub samples: u32,
}

pub struct Tracer<'a> {
    pub scene: &'a Scene,
    pub camera: &'a Camera,
    pub bvh: &'a Bvh<f64, 3>,
}

impl Tracer<'_> {
    fn get_current_ray(ray: &Ray, hit_info: &[HitInfo]) -> Ray {
        if let Some(last_hit) = hit_info.last() {
            last_hit.bounced.clone()
        } else {
            ray.clone()
        }
    }

    fn process_bounce(&self, ray: Ray) -> Option<HitInfo> {
        let current_ray = Rc::new(ray);
        let intersections: Vec<&Box<SceneObject>> =
            self.scene.intersect(self.bvh, Rc::clone(&current_ray));

        if intersections.is_empty() {
            return None;
        }

        let mut closest_distance = f64::INFINITY;
        let mut hit_info: Option<HitInfo> = None;

        for object in intersections {
            let geometry = &object.geometry;
            let material = &object.material;

            if let Some((hit_point, normal)) = geometry.intersect(&current_ray) {
                let dist = distance(&self.camera.position, &hit_point);
                if dist < closest_distance {
                    closest_distance = dist;
                    let new_bounce = material.bounce(&current_ray, hit_point, normal);

                    hit_info = Some(HitInfo {
                        incoming: *current_ray,
                        hit_point,
                        normal,
                        bounced: new_bounce,
                        object: Box::new(object.as_ref()),
                    });
                }
            }
        }

        hit_info
    }

    fn sample(&self, pixel: &PixelCoord<u32>, max_bounces: u32) -> Color<f64> {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let mut hit_info = Vec::new();
        let initial_ray = self.camera.get_ray(*pixel);

        for _bounces in 0..max_bounces {
            let ray = Tracer::get_current_ray(&initial_ray, &hit_info);
            if let Some(info) = self.process_bounce(ray) {
                hit_info.push(info);
            }
        }

        let final_ray = Tracer::get_current_ray(&initial_ray, &hit_info);
        let norm = final_ray.direction.normalize();
        let a = 0.5 * (norm.y + 1.0);
        color.x = 255.0 * (1.0 - a) + 0.5 * 255.0 * a;
        color.y = 255.0 * (1.0 - a) + 0.7 * 255.0 * a;
        color.z = 255.0 * (1.0 - a) + 1.0 * 255.0 * a;

        for hit in hit_info.iter() {
            color = hit
                .object
                .material
                .add_color(&hit.incoming, hit.hit_point, hit.normal, &color);
        }

        color
    }

    pub fn get_pixel(&self, pixel: &PixelCoord<u32>, parameters: &RenderParameters) -> Color<f64> {
        let samples: Vec<Color<f64>> = (0..parameters.samples)
            .into_par_iter() // Use parallel iterator
            .map(|_| self.sample(pixel, parameters.max_bounces)) // Sample for each iteration
            .collect(); // Collect results into a vector

        // Sum up the colors from all samples
        let mut color = Color::new(0.0, 0.0, 0.0);
        for sample in samples {
            color.x += sample.x;
            color.y += sample.y;
            color.z += sample.z;
        }

        // Average the color
        color.x /= parameters.samples as f64;
        color.y /= parameters.samples as f64;
        color.z /= parameters.samples as f64;

        color
    }
}
