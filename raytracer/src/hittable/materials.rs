use super::*;
use crate::color::Color;
use crate::math::{random_normalized, Vec3, R};
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: R },
    Dielectric { ir: R },
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
                // Degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = hit_record.normal;
                }

                let attenuation = *albedo;
                let scattered = Ray::new(hit_record.point, scatter_direction);

                Some((attenuation, scattered))
            }
            Material::Metal { albedo, fuzz } => {
                let fuzz = {
                    if *fuzz < 1. {
                        *fuzz
                    } else {
                        1.
                    }
                };

                let reflected = ray_in.direction().unit_vector().reflect(hit_record.normal);
                let scattered = Ray::new(
                    hit_record.point,
                    reflected + fuzz * Vec3::random_in_unit_sphere(),
                );
                let attenuation = albedo;

                if scattered.direction().dot(hit_record.normal) > 0. {
                    Some((*attenuation, scattered))
                } else {
                    None
                }
            }
            Material::Dielectric { ir } => {
                let attenuation = Color::new(1., 1., 1., 1.);
                let refraction_ratio = {
                    if hit_record.front_face {
                        1. / ir
                    } else {
                        *ir
                    }
                };

                let unit_dir = ray_in.direction().unit_vector();
                let cos_theta = { (-unit_dir).dot(hit_record.normal).min(1.) };
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.;

                let dir = {
                    if cannot_refract
                        || reflectance(cos_theta, refraction_ratio) > random_normalized()
                    {
                        unit_dir.reflect(hit_record.normal)
                    } else {
                        unit_dir.refract(hit_record.normal, refraction_ratio)
                    }
                };

                let scattered = Ray::new(hit_record.point, dir);

                Some((attenuation, scattered))
            }
        }
    }
}

fn reflectance(cos: R, ref_idx: R) -> R {
    // Schlick's approximation for reflectance
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;

    r0 + (1. - r0) * ((1. - cos).powf(5.))
}
