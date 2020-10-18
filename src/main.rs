use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::{random_double, write_color};
use crate::vec3::{Color, Point3, Vec3};
use rayon::prelude::*;
use std::f64::INFINITY;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

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
const SAMPLES_PER_PIXEL: u16 = 50;

const MAX_DEPTH: u16 = 50;

fn main() {
    // World
    // let mut world = HittableList::default();

    // let sphere_one =
    //     Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)) as Box<dyn Hittable + Send>;
    // let sphere_two =
    //     Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)) as Box<dyn Hittable + Send>;

    // world.add(sphere_one);
    // world.add(sphere_two);

    // Array holding the pixel values
    let mut pixel_vec = Arc::new(Mutex::new(vec![
        "".parse().unwrap();
        (IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32)
            as usize
    ]));
    //let mut pixel_vec: Vec<String> = vec!["".parse().unwrap(); (IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32) as usize];
    // let mut pixel_vec = Rc::new(Mutex::new(vec![
    //     "".parse().unwrap();
    //     (IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32)
    //         as usize
    // ]));

    let mut camera = Camera::new();
    /*
    pixel_vec.par_iter_mut().enumerate().for_each(|(idx, pixel)| {
        *pixel = write_pixel_for_index(&idx, &world)
    });*/

    // for j in 0..IMAGE_HEIGHT {
    //     threads.push(thread::spawn({
    //         let clone = Arc::clone(&pixel_vec);
    //         move || {
    //             let mut world = HittableList::default();
    //             let sphere_one =
    //                 Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)) as Box<dyn Hittable + Send>;
    //             let sphere_two =
    //                 Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)) as Box<dyn Hittable + Send>;

    //             world.add(sphere_one);
    //             world.add(sphere_two);
    //             write_pixel_in_vec_for_given_range(&mut clone.lock().unwrap(), &world, j);
    //         }
    //     }));
    // }
    // for t in threads {
    //     t.join().unwrap();
    // }

    /*(0..IMAGE_HEIGHT).into_par_iter().for_each(|j| {
        write_pixel_in_vec_for_given_range(&mut clone.lock().unwrap(), &world, j);
    });*/

    // let par_arrray = (0..IMAGE_HEIGHT).collect::<Vec<u16>>();

    // par_arrray.par_chunks_mut(10).for_each(|j| {
    //     write_pixel_in_vec_for_given_range(&mut pixel_vec, &world, j);
    // });

    //print_pixels(&mut pixel_vec.lock().unwrap());

    let mut handles = vec![];
    for thread_idx in 0..10 {
        let idx_part = IMAGE_HEIGHT / 10;
        let start_idx = thread_idx * idx_part;
        let end_idx = start_idx + idx_part;
        let pixel_vec = Arc::clone(&pixel_vec);
        let mut world = HittableList::default();

        let sphere_one =
            Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)) as Box<dyn Hittable + Send>;
        let sphere_two = Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))
            as Box<dyn Hittable + Send>;

        world.add(sphere_one);
        world.add(sphere_two);

        let handle = thread::spawn(move || {
            for j in start_idx..end_idx {
                write_pixel_in_vec_for_given_range(&mut pixel_vec.lock().unwrap(), &world, j);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    print_pixels(&mut pixel_vec.lock().unwrap());
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
        let pos = (line_nbr as usize * IMAGE_WIDTH as usize + i as usize);
        pixel_vec[pos] = write_color(pixel_color, SAMPLES_PER_PIXEL)
    }
}

fn write_pixel_for_index(idx: &usize, world: &HittableList) -> String {
    let mut camera = Camera::new();
    let (x, y) = get_x_y_from_index(idx);
    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    for _s in 0..SAMPLES_PER_PIXEL {
        let u = (f64::from(x) + random_double()) / f64::from(IMAGE_WIDTH - 1);
        let v = (f64::from(y) + random_double()) / f64::from(IMAGE_HEIGHT - 1);
        let r = camera.get_ray(u, v);
        pixel_color = pixel_color + ray_color(r, &world, MAX_DEPTH)
    }
    let pos = y as usize * IMAGE_WIDTH as usize + x as usize;
    write_color(pixel_color, SAMPLES_PER_PIXEL)
}

fn get_x_y_from_index(idx: &usize) -> (u16, u16) {
    let height_pos = idx / IMAGE_HEIGHT as usize;
    let width_pos = idx - (height_pos * IMAGE_WIDTH as usize);
    (height_pos as u16, width_pos as u16)
}

fn print_pixels(pixel_vec: &mut Vec<String>) {
    // Render
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for pixel in pixel_vec.iter().rev() {
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
