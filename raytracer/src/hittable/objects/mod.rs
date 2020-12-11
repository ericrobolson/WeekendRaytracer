use crate::color::Color;
use crate::math::{random_normalized, random_range, Vec3, R};
use crate::ray::Ray;

use super::*;
use materials::Material;

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

    pub fn random_scene() -> Self {
        let mut world = Self::new();

        let ground_material = Material::Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5, 1.),
        };

        world.add_item(Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            ground_material,
        ));

        // random items
        {
            for a in -11..11 {
                for b in -11..11 {
                    let a = a as R;
                    let b = b as R;

                    let choose_mat = random_normalized();

                    let center = Vec3::new(
                        a + 0.9 * random_normalized(),
                        0.2,
                        b + 0.9 * random_normalized(),
                    );
                    if (center - Vec3::new(4., 0.2, 0.)).len() > 0.9 {
                        if choose_mat < 0.8 {
                            let albedo = Vec3::random() * Vec3::random();
                            let albedo = Color::new(albedo.x, albedo.y, albedo.z, 1.);
                            let mat = Material::Lambertian { albedo };
                            world.add_item(Sphere::new(center, 0.2, mat));
                        } else if choose_mat < 0.95 {
                            let albedo = Vec3::random_range(0.5, 1.);
                            let albedo = Color::new(albedo.x, albedo.y, albedo.z, 1.);
                            let fuzz = random_range(0.5, 1.);
                            let mat = Material::Metal { albedo, fuzz };
                            world.add_item(Sphere::new(center, 0.2, mat));
                        } else {
                            let mat = Material::Dielectric { ir: 1.5 };
                            world.add_item(Sphere::new(center, 0.2, mat));
                        }
                    }
                }
            }
        }

        let material1 = Material::Dielectric { ir: 1.5 };
        world.add_item(Sphere::new(Vec3::new(0., 1., 0.), 1., material1));

        let material2 = Material::Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1, 1.),
        };
        world.add_item(Sphere::new(Vec3::new(-4., 1., 0.), 1., material2));

        let material3 = Material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5, 1.),
            fuzz: 0.,
        };
        world.add_item(Sphere::new(Vec3::new(4., 1., 0.), 1., material3));

        world
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: R, t_max: R) -> Option<HitRecord> {
        let mut temp_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for item in &self.items {
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
        let a = ray.direction().len_squared();
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
