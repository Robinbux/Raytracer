use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub struct Lambertian {
    pub(crate) albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
