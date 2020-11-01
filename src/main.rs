use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::{random_double, write_color};
use crate::vec3::{Color, Point3};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use std::cmp;
use std::f64::INFINITY;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod camera;
mod hittable;
mod material;
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
const CHUNK_SIZE: u16 = (IMAGE_HEIGHT / 10) * IMAGE_WIDTH;

const MAX_DEPTH: u16 = 50;

fn main() {
    // World
    let world = setup_scene();

    // Array holding the pixel values
    // let mut pixel_vec: Vec<String> =
    //     vec!["".parse().unwrap(); (IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32) as usize];
    //Vec::with_capacity((IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32) as usize);

    // for j in (0..IMAGE_HEIGHT).rev() {
    //     write_pixel_in_vec_for_given_range(&mut pixel_vec, &world, j);
    // }
    //
    // print_pixels(&mut pixel_vec);

    let mut pixel_vec = Arc::new(Mutex::new(vec![
        String::from("");
        (IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32)
            as usize
    ]));
    eprintln!("IMAGE WIDTH: {}", IMAGE_WIDTH);
    eprintln!("IMAGE HEIGHT: {}", IMAGE_HEIGHT);
    eprintln!("CHUNK SIZE: {}", CHUNK_SIZE);
    let now = Instant::now();
    pixel_vec
        .lock()
        .unwrap()
        .par_chunks_mut((CHUNK_SIZE).into())
        .enumerate()
        .for_each(|(chunk_idx, slice)| {
            let chunk_start_pos = chunk_idx_to_start_pixel_pos(chunk_idx);
            let chunk_end_pos = cmp::min(
                chunk_start_pos + CHUNK_SIZE as usize,
                (IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32) as usize,
            );
            eprintln!("-------------------------
Chunk Idx: {}
Slice len: {}
chunk_start_pos: {}
chunk_end_pos: {}
-------------------------", chunk_idx, slice.len(), chunk_start_pos, chunk_end_pos);

            for (idx, pixel_nbr) in (chunk_start_pos..chunk_end_pos).enumerate() {
                slice[idx] = color_for_pixel(&world, pixel_nbr as u32);
            }
        });
    let d = now.elapsed();
    print_pixels(&mut pixel_vec.lock().unwrap());
    eprintln!("took {:?}", d);
    eprintln!("Done\n");
}

fn chunk_idx_to_start_pixel_pos(chunk_idx: usize) -> usize {
    CHUNK_SIZE as usize * chunk_idx
}

fn color_for_pixel(world: &HittableList, pixel_nbr: u32) -> String {
    let camera = Camera::new();
    let line_nbr = pixel_to_line_nbr(pixel_nbr) as u16;
    let image_width_pos = pixel_nbr_to_image_width_pos(pixel_nbr);
    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    for _s in 0..SAMPLES_PER_PIXEL {
        let u = (f64::from(image_width_pos) + random_double()) / f64::from(IMAGE_WIDTH - 1);
        let v = (f64::from(line_nbr) + random_double()) / f64::from(IMAGE_HEIGHT - 1);
        let r = camera.get_ray(u, v);
        pixel_color = pixel_color + ray_color(r, &world, MAX_DEPTH)
    }
    write_color(pixel_color, SAMPLES_PER_PIXEL)
}

fn pixel_to_line_nbr(pixel_nbr: u32) -> usize {
    (IMAGE_HEIGHT - (pixel_nbr / IMAGE_WIDTH as u32) as u16) as usize
}

fn pixel_nbr_to_image_width_pos(pixel_nbr: u32) -> u16 {
    (pixel_nbr % IMAGE_WIDTH as u32) as u16
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
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.2);

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
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        Arc::new(material_left),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_right),
    )));

    world
}
