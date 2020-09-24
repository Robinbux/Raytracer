use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};
use std::f64::INFINITY;

mod color;
mod hittable;
mod math_constants;
mod ray;
mod sphere;
mod vec3;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;

// Camera
const VIEWPOINT_HEIGHT: f64 = 2.0;
const VIEWPOINT_WIDTH: f64 = ASPECT_RATIO * VIEWPOINT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

const ORIGIN: Point3 = Point3::new(0 as f64, 0 as f64, 0 as f64);
const CENTER: Point3 = Point3::new(0 as f64, 0 as f64, -1 as f64);
const HORIZONTAL: Vec3 = Vec3::new(VIEWPOINT_WIDTH, 0.0, 0.0);
const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPOINT_HEIGHT, 0.0);
const FOCAL: Vec3 = Vec3::new(0.0, 0.0, FOCAL_LENGTH);

fn main() {
    // Camera
    let LOWER_LEFT_CORNER: Point3 = ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - FOCAL; // Unable to make const...

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Render
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
            let v = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);
            let r = Ray::new(
                ORIGIN,
                LOWER_LEFT_CORNER + u * HORIZONTAL + v * VERTICAL - ORIGIN,
            );
            let pixel_color = ray_color(r, &world);
            write_color(pixel_color);
        }
    }
    eprintln!("Done\n");
}

// Linear interpolation between blue and white, bases on t
fn ray_color(ray: Ray, world: &HittableList) -> Color {
    let rec = HitRecord::default();
    let hit_record_option = world.hit(ray, 0.0, INFINITY);
    if hit_record_option.is_some() {
        return 0.5 * (hit_record_option.unwrap().normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.dir.unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.orig - center;
    let a = ray.dir.length_squared();
    let half_b = oc.dot(ray.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
