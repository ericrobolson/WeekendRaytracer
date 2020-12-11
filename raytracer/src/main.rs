use image::{ImageBuffer, Rgba, RgbaImage};
use rayon::prelude::*;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod math;
pub mod ray;
pub mod time;

use math::{Vec3, INFINITY, PI, R};

use color::Color;
use hittable::{
    materials::Material,
    objects::{Sphere, World},
    Hittable,
};
use ray::Ray;

fn main() {
    // Screen
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 480;
    let image_height: u32 = ((image_width as f32) / aspect_ratio) as u32;

    // Image creation
    let samples_per_pixel = 100;
    let mut img: RgbaImage = ImageBuffer::new(image_width, image_height);
    let max_depth = 50;

    // Camera
    let camera = {
        let eye = Vec3::new(3., 3., 2.);
        let target = Vec3::new(0., 0., -1.);
        let up_dir = Vec3::new(0., 1., 0.);

        let v_fov_degrees = 20.;
        let depth_of_field = (eye - target).len();
        let aperture = 2.;

        camera::Camera::new(
            eye,
            target,
            up_dir,
            v_fov_degrees,
            aspect_ratio,
            aperture,
            depth_of_field,
        )
    };

    // World

    let world = {
        let mut world = World::new();
        let mat_ground = Material::Lambertian {
            albedo: Color::new(0.8, 0.8, 0.0, 1.),
        };

        let mat_center = Material::Lambertian {
            albedo: Color::new(0.1, 0.2, 0.5, 1.),
        };

        let mat_left = Material::Dielectric { ir: 1.5 };

        let mat_back_up = Material::Metal {
            albedo: Color::new(0.4, 0.4, 0.2, 1.),
            fuzz: 0.,
        };

        let mat_right = Material::Metal {
            albedo: Color::new(0.8, 0.6, 0.2, 1.),
            fuzz: 0.1,
        };

        world.add_item(Sphere::new(Vec3::new(0., -100.5, -1.), 100., mat_ground));

        world.add_item(Sphere::new(Vec3::new(0., 0., -1.), 0.5, mat_center));
        world.add_item(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, mat_left));
        world.add_item(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right));

        world.add_item(Sphere::new(Vec3::new(0.0, 1.0, -1.4), 0.6, mat_back_up));

        world
    };

    // Render loop
    let clock = time::Clock::new();

    let samples = {
        let mut s = vec![];
        for _sample in 0..samples_per_pixel {
            s.push(_sample);
        }
        s
    };

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
            let mut color = Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.,
            };

            let colors: Vec<Color> = samples
                //.par_iter() // NOTE: par_iter() doesn't seem to benefit this case.
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let (u2, v2) = {
                        if samples_per_pixel == 1 {
                            (0., 0.)
                        } else {
                            (math::random_normalized(), math::random_normalized())
                        }
                    };

                    let u = ((i as R) + u2) / ((image_width - 1) as R);
                    let v = ((j as R) + v2) / ((image_height - 1) as R);

                    let ray = camera.get_ray(u, v);

                    let c = ray_color(&ray, &world, max_depth);
                    c
                })
                .collect();

            colors.iter().for_each(|c| {
                color += *c;
            });

            (i, j, color.from_samples(samples_per_pixel))
        })
        .collect();

    // Save img
    for (i, j, color) in colors {
        img.put_pixel(i, (image_height - 1) - j, Rgba(color.into()));
    }

    img.save("test.png").unwrap();
    println!("Run time: {:?}.", clock.elapsed());
}

fn ray_color(ray: &Ray, world: &World, depth: i32) -> Color {
    let miss_color = Color::new(0., 0., 0., 1.);
    if depth <= 0 {
        return miss_color;
    }
    let min_hit = 0.001;
    match world.hit(ray, min_hit, INFINITY) {
        Some(hr) => match hr.material.scatter(ray, &hr) {
            Some((attenuation, scattered_dir)) => {
                let mut col = attenuation * ray_color(&scattered_dir, world, depth - 1);
                col.a = 1.0;
                return col;
            }
            None => {
                return miss_color;
            }
        },
        None => {}
    }

    // Sky
    let unit_dir = ray.direction().unit_vector();
    let t = 0.5 * (unit_dir.y + 1.);

    let v_color = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0);
    Color::from_vec3(v_color, 255)
}
