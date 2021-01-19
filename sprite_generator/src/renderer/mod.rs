use image::{ImageBuffer, Rgba, RgbaImage};
use rayon::prelude::*;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod math;
pub mod ray;

pub use camera::CameraSettings;
use color::Color;
use hittable::{
    materials::Material,
    objects::{Mesh, Sphere, World},
    Hittable,
};
pub use math::{Vec3, INFINITY, R};
use ray::Ray;

pub use camera::Perspective;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShadingModes {
    Diffuse,
    Normal,
}

pub struct SpriteRenderer {
    pub shading_mode: ShadingModes,
    pub image_width: u32,
    pub image_height: u32,
    pub camera_settings: CameraSettings,
    pub mesh_file: String,
    pub blacken_normal_map: bool,
}

impl SpriteRenderer {
    pub fn render(&self) -> RgbaImage {
        render(
            self.image_width,
            self.image_height,
            self.shading_mode,
            self.blacken_normal_map,
            self.camera_settings,
            self.mesh_file.clone(),
        )
    }
}

fn render(
    image_width: u32,
    image_height: u32,
    shading_mode: ShadingModes,
    blacken_normal_map: bool,
    camera_settings: CameraSettings,
    mesh_file: String,
) -> RgbaImage {
    // Screen
    let aspect_ratio = (image_width as R) / (image_height as R);

    // Image creation
    let mut img: RgbaImage = ImageBuffer::new(image_width, image_height);

    // World
    let world = {
        // Scale objects so that their width is less than 1.0
        let mut world = World::new();
        let size_scale = 1.;
        /*
                let pos = size_scale * Vec3::new(0.0, 0.0, -1.);
                world.add_item(Sphere::new(
                    pos,
                    0.3 * size_scale,
                    Material::Lambertian {
                        albedo: Color::new(1., 0., 0.0, 1.),
                    },
                ));

                let pos = size_scale * Vec3::new(1.0, 0.0, -1.);
                world.add_item(Sphere::new(
                    pos,
                    0.3 * size_scale,
                    Material::Lambertian {
                        albedo: Color::new(1., 1., 0.0, 1.),
                    },
                ));

                let pos = size_scale * Vec3::new(1.0, 1.0, -1.);
                world.add_item(Sphere::new(
                    pos,
                    0.6 * size_scale,
                    Material::Lambertian {
                        albedo: Color::new(1., 0., 1.0, 1.),
                    },
                ));
        */
        let mesh = Mesh::from_obj(mesh_file);
        world.add_item(mesh);

        world
    };

    // Camera
    let camera = camera_settings.into_camera(aspect_ratio);

    // Render loop
    let pixels = {
        let mut pixels = vec![];
        for j in 0..image_height {
            for i in 0..image_width {
                pixels.push((i, j));
            }
        }
        pixels
    };

    let colors: Vec<(u32, u32, Color)> = pixels
        .par_iter()
        .map(|(i, j)| {
            let i = *i;
            let j = *j;

            let u = (i as R) / ((image_width - 1) as R);
            let v = (j as R) / ((image_height - 1) as R);

            let ray = camera.ray(u, v);

            let color = ray_color(&ray, &world, shading_mode);

            (i, j, color)
        })
        .collect();

    // Save img
    for (i, j, color) in colors {
        let color = {
            if shading_mode == ShadingModes::Normal {
                if color.a == 0. && blacken_normal_map {
                    Color::new(0., 0., 0., 1.)
                } else {
                    color
                }
            } else {
                let mut color = color;
                color.r /= 2.;
                color.g /= 2.;
                color.b /= 2.;
                color
            }
        };

        img.put_pixel(i, (image_height - 1) - j, Rgba(color.into()));
    }

    img
}

fn ray_color(ray: &Ray, world: &World, shading_mode: ShadingModes) -> Color {
    let min_hit = 0.0001;
    match world.hit(ray, min_hit, INFINITY) {
        Some(hr) => match shading_mode {
            ShadingModes::Diffuse => {
                return hr.material.color();
            }
            ShadingModes::Normal => {
                // Now that we have the normal, orient it to the camera.
                // map x,y,z from -1..1 to 0..1
                // println!("Normal: {:?}", hr.normal);

                let normal = (hr.normal + Vec3::one()) / 2.;

                return Color::from_vec3(normal, 1.);
            }
        },
        None => {}
    }

    Color::new(0., 0., 0., 0.)
}
