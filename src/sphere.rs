use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

use std::sync::Arc;

pub struct Sphere {
    pub(crate) center: Point3,
    pub(crate) radius: f64,
    pub(crate) material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            for t in &[(-half_b - root) / a, (-half_b + root) / a] {
                if t < &t_max && t > &t_min {
                    let t = *t;
                    let p = ray.at(t);
                    let outward_normal = (p - self.center) / self.radius;
                    let mut hit_record =
                        HitRecord::new(p, outward_normal, Arc::clone(&self.material), t, true);
                    hit_record.set_face_normal(ray, outward_normal);
                    return Some(hit_record);
                }
            }
        }
        return None;
    }
}
