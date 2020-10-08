use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::{random_double, write_color};
use crate::vec3::{Color, Point3, Vec3};
use std::f64::INFINITY;

mod camera;
mod hittable;
mod math_constants;
mod ray;
mod sphere;
mod util;
mod vec3;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
const SAMPLES_PER_PIXEL: u16 = 100;

const MAX_DEPTH: u16 = 50;

fn main() {
    // Camera
    let mut camera = Camera::new();

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Render
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (f64::from(i) + random_double()) / f64::from(IMAGE_WIDTH - 1);
                let v = (f64::from(j) + random_double()) / f64::from(IMAGE_HEIGHT - 1);
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, MAX_DEPTH)
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL)
        }
    }
    eprintln!("Done\n");
}

// Linear interpolation between blue and white, bases on t
fn ray_color(ray: Ray, world: &HittableList, depth: u16) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.0, INFINITY) {
        Some(hit_record) => {
            let target: Point3 = hit_record.p + hit_record.normal + Vec3::random_in_unit_sphere();
            0.5 * ray_color(
                Ray::new(hit_record.p, target - hit_record.p),
                world,
                depth - 1,
            )
        }
        None => {
            let unit_direction = ray.dir.unit();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.dir.length_squared();
    let half_b = oc.dot(ray.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    };
}
