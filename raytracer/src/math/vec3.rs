use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: R,
    pub y: R,
    pub z: R,
}

impl Vec3 {
    pub fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn new(x: R, y: R, z: R) -> Self {
        Self { x, y, z }
    }

    pub fn len_squared(&self) -> R {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> R {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, v: Self) -> R {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: (self.y * other.z - self.z * other.y),
            y: (self.z * other.x - self.x * other.z),
            z: (self.x * other.y - self.y * other.x),
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn random() -> Self {
        Self {
            x: random_normalized(),
            y: random_normalized(),
            z: random_normalized(),
        }
    }

    pub fn random_range(min: R, max: R) -> Self {
        Self {
            x: random_range(min, max),
            y: random_range(min, max),
            z: random_range(min, max),
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1., 1.);
            if p.len_squared() >= 1. {
                continue;
            }

            return p;
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new(random_range(-1., 1.), random_range(-1., 1.), 0.);
            if p.len_squared() >= 1. {
                continue;
            }

            return p;
        }
    }

    pub fn near_zero(&self) -> bool {
        let min = 1e-8;
        self.x.abs() < min && self.y.abs() < min && self.z.abs() < min
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - 2. * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: Vec3, etai_over_etat: R) -> Self {
        let uv = *self;
        let cos_theta = (-uv).dot(normal).min(1.);
        let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
        let r_out_paralell = {
            let n = (1. - r_out_perp.len_squared()).sqrt();
            (-n) * normal
        };
        r_out_perp + r_out_paralell
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for R {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl std::ops::Mul<R> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: R) -> Vec3 {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::MulAssign<R> for Vec3 {
    fn mul_assign(&mut self, rhs: R) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Div<R> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: R) -> Vec3 {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::DivAssign<R> for Vec3 {
    fn div_assign(&mut self, rhs: R) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_default_returns_000() {
        let actual = Vec3::default();

        assert_eq!(0., actual.x);
        assert_eq!(0., actual.y);
        assert_eq!(0., actual.z);

        let expected = Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_new_returns_expected() {
        let x = 2.3;
        let y = 3382.90001;
        let z = -0.00001;
        let actual = Vec3::new(x, y, z);
        let expected = Vec3 { x, y, z };

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_neg_returns_expected() {
        let x = 2.3;
        let y = 3382.90001;
        let z = -0.00001;
        let actual = -(Vec3::new(x, y, z));

        let x = -x;
        let y = -y;
        let z = -z;
        let expected = Vec3 { x, y, z };

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_add_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let x1 = 34.2;
        let y1 = -2090.012;
        let z1 = 3.02;

        let actual = Vec3::new(x0, y0, z0) + Vec3::new(x1, y1, z1);

        let expected = Vec3::new(x0 + x1, y0 + y1, z0 + z1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_addassign_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let x1 = 34.2;
        let y1 = -2090.012;
        let z1 = 3.02;

        let mut actual = Vec3::new(x0, y0, z0);
        actual += Vec3::new(x1, y1, z1);

        let expected = Vec3::new(x0 + x1, y0 + y1, z0 + z1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_sub_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let x1 = 34.2;
        let y1 = -2090.012;
        let z1 = 3.02;

        let actual = Vec3::new(x0, y0, z0) - Vec3::new(x1, y1, z1);

        let expected = Vec3::new(x0 - x1, y0 - y1, z0 - z1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_subassign_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let x1 = 34.2;
        let y1 = -2090.012;
        let z1 = 3.02;

        let mut actual = Vec3::new(x0, y0, z0);
        actual -= Vec3::new(x1, y1, z1);

        let expected = Vec3::new(x0 - x1, y0 - y1, z0 - z1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_mul_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let x1 = 34.2;
        let y1 = -2090.012;
        let z1 = 3.02;

        let actual = Vec3::new(x0, y0, z0) * Vec3::new(x1, y1, z1);

        let expected = Vec3::new(x0 * x1, y0 * y1, z0 * z1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_dot_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let x1 = 34.2;
        let y1 = -2090.012;
        let z1 = 3.02;

        let actual = Vec3::new(x0, y0, z0).dot(Vec3::new(x1, y1, z1));
        let expected = x0 * x1 + y0 * y1 + z0 * z1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_cross_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let x1 = 34.2;
        let y1 = -2090.012;
        let z1 = 3.02;

        let actual = Vec3::new(x0, y0, z0).cross(Vec3::new(x1, y1, z1));
        let expected = Vec3 {
            x: (y0 * z1 - z0 * y1),
            y: (z0 * x1 - x0 * z1),
            z: (x0 * y1 - y0 * x1),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_unit_vector_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;

        let vec = Vec3::new(x0, y0, z0);
        let actual = vec.unit_vector();
        let expected = vec / vec.len();

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_mulr_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let mul = 2382.10191;

        let actual = Vec3::new(x0, y0, z0) * mul;
        let expected = Vec3::new(x0 * mul, y0 * mul, z0 * mul);
        assert_eq!(expected, actual);

        let actual = mul * Vec3::new(x0, y0, z0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_mulassign_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let mul = 2.381;

        let mut actual = Vec3::new(x0, y0, z0);
        actual *= mul;

        let expected = Vec3::new(x0 * mul, y0 * mul, z0 * mul);

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_divr_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let div = 2382.10191;

        let actual = Vec3::new(x0, y0, z0) / div;
        let expected = Vec3::new(x0 / div, y0 / div, z0 / div);
        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_divassign_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;
        let div = 2.381;

        let mut actual = Vec3::new(x0, y0, z0);
        actual /= div;

        let expected = Vec3::new(x0 / div, y0 / div, z0 / div);

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_len_squared_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;

        let actual = Vec3::new(x0, y0, z0).len_squared();
        let expected = x0 * x0 + y0 * y0 + z0 * z0;

        assert_eq!(expected, actual);
    }

    #[test]
    fn vec3_len_returns_expected() {
        let x0 = 2.3;
        let y0 = 3382.90001;
        let z0 = -0.00001;

        let actual = Vec3::new(x0, y0, z0).len();
        let expected = (x0 * x0 + y0 * y0 + z0 * z0).sqrt();

        assert_eq!(expected, actual);
    }
}
