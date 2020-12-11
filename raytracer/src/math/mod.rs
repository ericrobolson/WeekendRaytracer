use rand::Rng;

mod vec3;
pub use vec3::Vec3;

pub type R = f32;
pub const INFINITY: R = R::MAX;
pub const PI: R = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: R) -> R {
    degrees * PI / 180.
}

pub fn random_normalized() -> R {
    random_range(0., 1.)
}

pub fn random_range(min: R, max: R) -> R {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

pub fn clamp(n: R, min: R, max: R) -> R {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}
