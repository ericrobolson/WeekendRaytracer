use crate::math::{Vec3, R};
use crate::ray::Ray;

use super::*;
use materials::Material;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: R, t_max: R) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub t: R,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: R, point: Vec3, outward_normal: Vec3, ray: &Ray, material: Material) -> Self {
        let front_face = ray.direction().dot(outward_normal) < 0.;
        let normal = {
            if front_face {
                outward_normal
            } else {
                -outward_normal
            }
        };

        Self {
            t,
            point,
            normal,
            front_face,
            material,
        }
    }
}
