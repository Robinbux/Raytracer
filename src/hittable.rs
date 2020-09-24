use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    // Might be wrong?
    pub fn hittable_list(&mut self, object: Box<dyn Hittable>) {
        self.add(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let temp_rec: HitRecord = HitRecord::default();
        let mut closest_so_far = t_max;
        let mut res = None;

        for object in &self.objects {
            let hit_record_option = object.hit(ray, t_min, closest_so_far);
            if hit_record_option.is_some() {
                res = hit_record_option;
                closest_so_far = temp_rec.t;
            }
        }
        return res;
    }
}
