use crate::camera::camera::Camera;
use crate::objects::hittable::{Hittable, HittableList};
use crate::utils::util::{random_double, write_color, write_pixels_to_file, random_double_in_range};
use crate::vec::vec3::{Color, Point3, Ray, Vec3};
use std::f64::INFINITY;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::cmp;
use crate::objects::material::{Lambertian, Metal, Dielectric};
use crate::objects::sphere::Sphere;
use rayon::slice::ParallelSliceMut;
use rayon::iter::{IndexedParallelIterator, ParallelIterator};

const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: u32 = 50;

pub struct Scene {
    world: HittableList,
    camera: Camera,
    image_width: u32,
    image_height: u32,
    aspect_ratio: f64,
    pixel_vec: Arc<Mutex<Vec<String>>>,
    chunk_size: u32
}

impl Scene {
    pub fn new(
        world: HittableList,
        camera: Camera,
        image_width: u32,
        aspect_ratio: f64,
    ) -> Scene {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let pixel_vec = Arc::new(Mutex::new(vec![
            String::from("");
            (image_width * image_height) as usize
        ]));
        let chunk_size = (image_height / 10) * image_width;
        Scene {
            world,
            camera,
            image_width,
            image_height,
            aspect_ratio,
            pixel_vec,
            chunk_size
        }
    }

    pub fn multithreadet_rendering(&self) {
        let now = Instant::now();
        self.pixel_vec
            .lock()
            .unwrap()
            .par_chunks_mut(self.chunk_size as usize)
            .enumerate()
            .for_each(|(chunk_idx, slice)| {
                let chunk_start_pos = self.chunk_idx_to_start_pixel_pos(chunk_idx);
                let chunk_end_pos = cmp::min(
                    chunk_start_pos + self.chunk_size as usize,
                    (self.image_width as u32 * self.image_height as u32) as usize,
                );
                println!(
                    "-------------------------\n\
                    Chunk Idx: {}\n\
                    Slice len: {}\n\
                    chunk_start_pos: {}\n\
                    chunk_end_pos: {}\n\
                    -------------------------",
                    chunk_idx,
                    slice.len(),
                    chunk_start_pos,
                    chunk_end_pos
                );

                for (idx, pixel_nbr) in (chunk_start_pos..chunk_end_pos).enumerate() {
                    slice[idx] = self.color_for_pixel(pixel_nbr as u32);
                }
            });
        let d = now.elapsed();
        write_pixels_to_file(&mut self.pixel_vec.lock().unwrap(), self.image_width, self.image_height);
        println!("took {:?}", d);
        println!("Done\n");
    }

    fn color_for_pixel(&self, pixel_nbr: u32) -> String {
        let line_nbr = self.pixel_to_line_nbr(pixel_nbr) as u32;
        let image_width_pos = self.pixel_nbr_to_image_width_pos(pixel_nbr);
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _s in 0..SAMPLES_PER_PIXEL {
            let u =
                (f64::from(image_width_pos) + random_double()) / f64::from(self.image_width - 1);
            let v = (f64::from(line_nbr) + random_double()) / f64::from(self.image_height - 1);
            let r = self.camera.get_ray(u, v);
            pixel_color = pixel_color + self.ray_color(r, MAX_DEPTH)
        }
        write_color(pixel_color, SAMPLES_PER_PIXEL as u16)
    }

    fn ray_color(&self, ray: Ray, depth: u32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match self.world.hit(ray, 0.0, INFINITY) {
            Some(hit_record) => {
                if let Some((scattered, attenuation)) =
                    hit_record.material.scatter(&ray, &hit_record)
                {
                    attenuation * self.ray_color(scattered, depth - 1)
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

    fn chunk_idx_to_start_pixel_pos(&self, chunk_idx: usize) -> usize {
        self.chunk_size as usize * chunk_idx
    }

    fn pixel_to_line_nbr(&self, pixel_nbr: u32) -> usize {
        (self.image_height - (pixel_nbr / self.image_width as u32) as u32) as usize
    }

    fn pixel_nbr_to_image_width_pos(&self, pixel_nbr: u32) -> u32 {
        (pixel_nbr % self.image_width as u32) as u32
    }

    #[allow(dead_code)]
    pub(crate) fn setup_complex_scene() -> Scene {
        let mut world = HittableList::default();

        let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
        world.add(Box::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(material_ground),
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_double();
                let center = Point3::new(
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let sphere = match choose_mat {
                        mat if mat < 0.8 => {
                            // Diffuse
                            let albedo = Color::random() * Color::random();
                            let sphere_material = Lambertian::new(albedo);
                            Box::new(Sphere::new(
                                Point3::new(0.0, -100.5, -1.0),
                                100.0,
                                Arc::new(sphere_material),
                            ))
                        }
                        mat if mat < 0.95 => {
                            // Metal
                            let albedo = Color::random_in_range(0.5, 1.0);
                            let fuzz = random_double_in_range(0.0, 0.5);
                            let sphere_material = Metal::new(albedo, fuzz);
                            Box::new(Sphere::new(
                                Point3::new(0.0, -100.5, -1.0),
                                100.0,
                                Arc::new(sphere_material),
                            ))
                        }
                        _ => {
                            // Glass
                            let albedo = Color::random_in_range(0.5, 1.0);
                            let fuzz = random_double_in_range(0.0, 0.5);
                            let sphere_material = Metal::new(albedo, fuzz);
                            Box::new(Sphere::new(
                                Point3::new(0.0, -100.5, -1.0),
                                100.0,
                                Arc::new(sphere_material),
                            ))
                        }
                    };

                    world.add(sphere);
                }
            }
        }

        let material_1 = Dielectric::new(1.5);
        world.add(Box::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Arc::new(material_1),
        )));

        let material_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
        world.add(Box::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Arc::new(material_2),
        )));

        let material_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
        world.add(Box::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Arc::new(material_3),
        )));

        let lookfrom = Point3::new(13.0, 2.0, 3.0);
        let lookat = Point3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.1;

        let aspect_ratio = 3.0 / 2.0;
        let image_width: u32 = 1200;

        let camera = Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            aspect_ratio,
            aperture,
            dist_to_focus,
        );

        Scene::new(world, camera, image_width, aspect_ratio)
    }

    #[allow(dead_code)]
    pub(crate) fn setup_basic_scene() -> Scene {
        let mut world = HittableList::default();

        let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
        let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
        let material_left = Dielectric::new(1.5);
        let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

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

        let lookfrom = Point3::new(-2.0, 2.0, 1.0);
        let lookat = Point3::new(0.0, 0.0, -1.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.1;

        let aspect_ratio = 16.0 / 9.0;
        let image_width: u32 = 400;

        let camera = Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            aspect_ratio,
            aperture,
            dist_to_focus,
        );

        Scene::new(world, camera, image_width, aspect_ratio)
    }
}