use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub struct Metal {
    pub(crate) albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(&r_in.dir.unit(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
