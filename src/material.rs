use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Vec3, Color};

use std::cmp::min;

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

// ----------------------------------------------------------------------
// ----- METAL -----
pub struct Metal {
    pub(crate) albedo: Color,
    pub(crate) fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(&r_in.dir.unit(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

// ----------------------------------------------------------------------
// ----- LAMBERTIAN -----
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

// ----------------------------------------------------------------------
// ----- DIELECTRIC -----
pub struct Dielectric {
    pub(crate) reflection_index: f64,
}

impl Dielectric {
    pub fn new(reflection_index: f64) -> Dielectric {
        Dielectric { reflection_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let reflection_ratio = match rec.front_face {
            true => 1.0 / self.reflection_index,
            false => self.reflection_index
        };

        let unit_direction = r_in.dir.unit();

        // No min function for f64 apparently...
        let mut cos_theta = -unit_direction.dot(&rec.normal);
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }

        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = match reflection_ratio * sin_theta > 1.0 {
            true => Vec3::reflect(&unit_direction, &rec.normal),
            false => Vec3::refract(&unit_direction, &rec.normal, reflection_ratio)
        };

        let scattered = Ray::new(rec.p, direction);

        Some((scattered, attenuation))
    }
}