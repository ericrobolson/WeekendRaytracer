use crate::math::{degrees_to_radians, Vec3, R};
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: R,
}

impl Camera {
    pub fn new(
        eye: Vec3,
        target: Vec3,
        up_dir: Vec3,
        v_fov_degrees: R,
        aspect_ratio: R,
        aperture: R,
        depth_of_field: R,
    ) -> Self {
        let theta = degrees_to_radians(v_fov_degrees);
        let h = (theta / 2.).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (eye - target).unit_vector();
        let u = (up_dir.cross(w)).unit_vector();
        let v = w.cross(u);

        let origin = eye;
        let horizontal = depth_of_field * viewport_width * u;
        let vertical = depth_of_field * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - depth_of_field * w;

        let lens_radius = depth_of_field / 2.;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: R, v: R) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
