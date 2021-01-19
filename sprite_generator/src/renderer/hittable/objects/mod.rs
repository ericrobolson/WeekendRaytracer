use crate::renderer::color::Color;
use crate::renderer::math::{random_normalized, Quaternion, Vec3, R};
use crate::renderer::Ray;

use super::*;
use materials::Material;
mod mesh;
pub use mesh::Mesh;

pub struct World {
    items: Vec<Box<dyn Hittable + Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn add_item<T>(&mut self, hittable: T)
    where
        T: Hittable + Sync + 'static,
    {
        self.items.push(Box::new(hittable));
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: R, t_max: R) -> Option<HitRecord> {
        let mut temp_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for (i, item) in self.items.iter().enumerate() {
            match item.hit(ray, t_min, t_max) {
                Some(hr) => {
                    if closest_so_far > hr.t {
                        closest_so_far = hr.t;
                        temp_record = Some(hr);
                    }
                }
                None => {}
            }
        }

        temp_record
    }
}

pub struct Sphere {
    center: Vec3,
    radius: R,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: R, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: R, t_max: R) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let half_b = oc.dot(ray.direction());
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            return None;
        }

        let point = ray.at(root);

        let rec = HitRecord::new(
            root,
            point,
            (point - self.center) / self.radius,
            ray,
            self.material,
        );

        Some(rec)
    }
}
