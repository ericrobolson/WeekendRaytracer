use super::*;

#[derive(Copy, Clone, Debug)]
struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    edge1: Vec3,
    edge2: Vec3,
    normal: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = (edge1.cross(edge2)).unit_vector();

        Self {
            v0,
            v1,
            v2,
            edge1,
            edge2,
            normal,
        }
    }

    pub fn center(&self) -> Vec3 {
        let center = (self.v0 + self.v1 + self.v2) / 3.;
        center
    }
}

pub struct Mesh {
    triangles: Vec<Triangle>,
    material: Material,
    center: Vec3,
}

impl Mesh {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Self {
        let triangle = Triangle::new(v0, v1, v2);
        Self {
            triangles: vec![triangle],
            center: triangle.center(),
            material,
        }
    }

    pub fn from_gltf(filename: String) -> Self {
        unimplemented!();
    }

    pub fn from_obj(filename: String) -> Self {
        use tobj;

        let mut triangles = vec![];
        // TODO: materials
        let (models, materials) = tobj::load_obj(&filename, true).unwrap();

        let mut centers = Vec3::new(0., 0., 0.);

        for model in models.iter() {
            let mesh = &model.mesh;

            let mut verts = vec![];
            for v in 0..mesh.positions.len() / 3 {
                let vertx = mesh.positions[3 * v];
                let verty = mesh.positions[3 * v + 1];
                let vertz = mesh.positions[3 * v + 2];

                verts.push(Vec3::new(vertx, verty, vertz));
            }

            for t in 0..verts.len() / 3 {
                let v0 = verts[3 * t];
                let v1 = verts[3 * t + 1];
                let v2 = verts[3 * t + 2];
                let triangle = Triangle::new(v0, v1, v2);
                centers += triangle.center();

                triangles.push(triangle);
            }
        }

        let center = centers / triangles.len() as f32;

        Self {
            center,
            triangles,
            material: Material::Lambertian {
                albedo: Color::new(0., 1., 1.0, 1.),
            },
        }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: R, t_max: R) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest_so_far = t_max;
        for triangle in &self.triangles {
            match intersect_ray_triangle_mt(t_min, ray, triangle) {
                Some(intersection) => {
                    let t = intersection.point - ray.origin();
                    let t = t.len();

                    let mut rec = HitRecord::new(
                        t,
                        intersection.point,
                        intersection.normal,
                        ray,
                        self.material,
                    );
                    // TODO: instead of all triangles, just use the closest

                    if t <= closest_so_far {
                        // Fixes degenerate case on isosphere, but seems wrong
                        if !rec.front_face {
                            rec.normal = -rec.normal;
                        }
                        closest_so_far = t;
                        hit = Some(rec);
                    }
                }
                None => {}
            }
        }
        return hit;
    }
}

struct TriangleRayIntersection {
    point: Vec3,
    normal: Vec3,
}

const EPSILON: R = 0.0000001;
fn intersect_ray_triangle_mt(
    t_min: R,
    ray: &Ray,
    triangle: &Triangle,
) -> Option<TriangleRayIntersection> {
    let h = ray.direction().cross(triangle.edge2);
    let a = triangle.edge1.dot(h);
    if a > -EPSILON && a < EPSILON {
        // Ray is parallel to triangle
        return None;
    }

    let f = 1. / a;
    let s = ray.origin() - triangle.v0;
    let u = f * s.dot(h);
    if u < 0. || u > 1. {
        return None;
    }
    let q = s.cross(triangle.edge1);
    let v = f * ray.direction().dot(q);
    if v < 0. || u + v > 1. {
        return None;
    }

    let t = f * triangle.edge2.dot(q);
    if t > EPSILON {
        let intersection = ray.origin() + ray.direction() * t;

        let normal = {
            //TODO: calculate normal map so it's always the map facing the camera. Right now it's just using the triangles.

            let q = Quaternion::from_direction(intersection, triangle.center());

            let normal = q.rotate_vec3(triangle.normal).unit_vector();

            let normal = triangle.normal;

            normal
        };

        return Some(TriangleRayIntersection {
            point: intersection,
            normal,
        });
    }

    None
}
