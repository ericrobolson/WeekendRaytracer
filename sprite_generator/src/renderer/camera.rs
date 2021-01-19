use super::math::{degrees_to_radians, Quaternion, Vec3, R};
use super::ray::Ray;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Perspective {
    Perspective { scale: R },
    Orthographic { scale: R },
}

#[derive(PartialEq, Debug, Copy, Clone, Deserialize, Serialize)]
pub struct CameraSettings {
    v_fov: R,
    eye: Vec3,
    target: Vec3,
    up_dir: Vec3,
    perspective: Perspective,
    focal_len: R,
}

impl CameraSettings {
    pub fn into_camera(&self, aspect_ratio: R) -> Camera {
        Camera::new(
            aspect_ratio,
            self.v_fov,
            self.eye,
            self.target,
            self.up_dir,
            self.perspective,
            self.focal_len,
        )
    }
}

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    perspective: Perspective,
    origin: Vec3,
    normal: Vec3,
}

impl Camera {
    fn new(
        aspect_ratio: R,
        v_fov: R,
        eye: Vec3,
        target: Vec3,
        up_dir: Vec3,
        perspective: Perspective,
        focal_len: R,
    ) -> Self {
        let theta = degrees_to_radians(v_fov);
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let (focal_len, scale) = match perspective {
            Perspective::Perspective { scale } => (focal_len, scale),
            Perspective::Orthographic { scale } => (-focal_len, scale),
        };

        let w = (eye - target).unit_vector();
        let u = up_dir.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = eye;
        let horizontal = (scale * viewport_width) * u;
        let vertical = (scale * viewport_height) * v;

        let focal_norm = w * focal_len;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focal_norm;

        Self {
            lower_left_corner,
            horizontal,
            vertical,
            perspective,
            origin,
            normal: -w,
        }
    }

    pub fn ray(&self, u: R, v: R) -> Ray {
        let ray = {
            match self.perspective {
                Perspective::Perspective { .. } => Ray::new(
                    self.origin,
                    self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
                ),
                Perspective::Orthographic { .. } => Ray::new(
                    self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
                    self.normal,
                ),
            }
        };

        ray
    }
}
