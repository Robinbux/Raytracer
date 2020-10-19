use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::{random_double, write_color};
use crate::vec3::{Color, Point3, Vec3};
use std::f64::INFINITY;
use std::sync::Arc;

mod camera;
mod hittable;
mod lambertian;
mod material;
mod math_constants;
mod metal;
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
    // World
    let mut world = setup_scene();

    // Array holding the pixel values
    let mut pixel_vec: Vec<String> =
        vec!["".parse().unwrap(); (IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32) as usize];
    //Vec::with_capacity((IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32) as usize);

    for j in (0..IMAGE_HEIGHT).rev() {
        write_pixel_in_vec_for_given_range(&mut pixel_vec, &world, j);
    }

    print_pixels(&mut pixel_vec);

    eprintln!("Done\n");
}

fn write_pixel_in_vec_for_given_range(
    pixel_vec: &mut Vec<String>,
    world: &HittableList,
    line_nbr: u16,
) {
    // Camera
    let mut camera = Camera::new();
    eprintln!("Scanlines remaining: {} ", line_nbr);
    for i in 0..IMAGE_WIDTH {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _s in 0..SAMPLES_PER_PIXEL {
            let u = (f64::from(i) + random_double()) / f64::from(IMAGE_WIDTH - 1);
            let v = (f64::from(line_nbr) + random_double()) / f64::from(IMAGE_HEIGHT - 1);
            let r = camera.get_ray(u, v);
            pixel_color = pixel_color + ray_color(r, &world, MAX_DEPTH)
        }
        let pos = (IMAGE_HEIGHT - 1 - line_nbr) as usize * IMAGE_WIDTH as usize + i as usize;
        pixel_vec[pos] = write_color(pixel_color, SAMPLES_PER_PIXEL)
    }
}

fn print_pixels(pixel_vec: &mut Vec<String>) {
    // Render
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for pixel in pixel_vec.iter() {
        println!("{}", pixel);
    }
}

// Linear interpolation between blue and white, bases on t
fn ray_color(ray: Ray, world: &HittableList, depth: u16) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.0, INFINITY) {
        Some(hit_record) => {
            if let Some((scattered, attenuation)) = hit_record.material.scatter(&ray, &hit_record) {
                attenuation * ray_color(scattered, world, depth - 1)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = ray.dir.unit();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn setup_scene() -> HittableList {
    let mut world = HittableList::default();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(material_ground),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(material_center),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_left),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_right),
    )));

    world
}
