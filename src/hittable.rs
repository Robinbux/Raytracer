use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) material: Arc<dyn Material>,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        material: Arc<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> HitRecord {
        HitRecord {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.dir.dot(&outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
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
    #[allow(dead_code)]
    pub fn hittable_list(&mut self, object: Box<dyn Hittable>) {
        self.add(object)
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut res = None;

        for object in &self.objects {
            let hit_record_option = object.hit(ray, t_min, closest_so_far);
            match hit_record_option {
                Some(rec) => {
                    closest_so_far = rec.t;
                    res = Some(rec);
                }
                None => {}
            }
        }
        res
    }
}
