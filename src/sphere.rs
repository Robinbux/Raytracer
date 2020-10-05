use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    pub(crate) center: Point3,
    pub(crate) radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            for t in &[(-half_b - root) / a, (-half_b + root) / a] {
                if t < &t_max && t > &t_min {
                    let mut hit_record = HitRecord::default();
                    hit_record.t = *t;
                    hit_record.p = ray.at(hit_record.t);
                    let outward_normal = (hit_record.p - self.center) / self.radius;
                    hit_record.set_face_normal(ray, outward_normal);
                    return Some(hit_record);
                }
            }
        }
        return None;
    }
}
