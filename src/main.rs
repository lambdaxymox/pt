extern crate cgmath;
extern crate rand;

mod ray;
mod hitable_list;
mod sphere;
mod camera;
mod material;
mod sample;

use rand::prelude::*;

use std::error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::f32;

use cgmath::{Vector3, Magnitude};
use camera::Camera;
use ray::Ray;
use sphere::Sphere;
use hitable_list::HitableList;
use material::*;


const MAX_DEPTH: u32 = 16;
const SAMPLES_PER_PIXEL: u32 = 128;


fn camera(width: u32, height: u32) -> Camera {
    let look_from = cgmath::vec3((3_f32, 3_f32, 2_f32));
    let look_at = cgmath::vec3((0_f32, 0_f32, -1_f32));
    let distance_to_focus = (look_from - look_at).magnitude();
    let aperture = 2_f32;
    let v_up = cgmath::vec3((0_f32, 1_f32, 0_f32));
    let v_fov = 20_f32;
    let aspect_ratio = (width as f32) / (height as f32);
    
    Camera::new(look_from, look_at, v_up, v_fov, aspect_ratio, aperture, distance_to_focus)
}

fn generate_scene(rng: &mut ThreadRng) -> HitableList {
    let mut world = HitableList::new();
    world.push(Box::new(
        Sphere::new(cgmath::vec3((0_f32, 0_f32, -1_f32)), 0.5, Material::lambertian(cgmath::vec3((0.1, 0.2, 0.5))))
    ));
    world.push(Box::new(
        Sphere::new(cgmath::vec3((0_f32, -100.5, -1_f32)), 100_f32, Material::lambertian(cgmath::vec3((0.8, 0.8, 0.0))))
    ));
    world.push(Box::new(
        Sphere::new(cgmath::vec3((1_f32, 0_f32, -1_f32)), 0.5, Material::metal(cgmath::vec3((0.8, 0.6, 0.2)), 0.3))
    ));
    world.push(Box::new(
        Sphere::new(cgmath::vec3((-1_f32, 0_f32, -1_f32)), 0.5, Material::dielectric(1.5))
    ));
    world.push(Box::new(
        Sphere::new(cgmath::vec3((-1_f32, 0_f32, -1_f32)), -0.45, Material::dielectric(1.5))
    ));

    world
}

#[inline]
fn component_multiply(v1: Vector3, v2: Vector3) -> Vector3 {
    cgmath::vec3((v1.x * v2.x, v1.y * v2.y, v1.z * v2.z))
}

fn color<H: Hitable>(ray: Ray, world: &H, rng: &mut ThreadRng, depth: u32) -> Vector3 {
    match world.hit(&ray, 0.001, f32::MAX) {
        Some(hit) => {    
            if depth < MAX_DEPTH {
                let scatter = hit.material.scatter(ray, &hit, rng);
                let col = color(scatter.ray, world, rng, depth + 1);
                return component_multiply(scatter.attenuation, col);
            } else {
                return cgmath::vec3((0_f32, 0_f32, 0_f32));
            }
        }
        None => {
            let unit_direction = ray.direction.normalize();
            let t = (unit_direction.y + 1_f32) * 0.5;
            return cgmath::vec3((1_f32, 1_f32, 1_f32)) * (1_f32 - t) + cgmath::vec3((0.5, 0.7, 1.0)) * t
        }
    }
}

fn to_bgra(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}

struct Image {
    width: u32,
    height: u32,
    data: Vec<u32>,
}

fn render(width: u32, height: u32, samples_per_pixel: u32, camera: Camera, world: HitableList) -> Image {
    let mut rng = rand::prelude::thread_rng();
    let mut data = vec![];
    for j in 0..height {
        for i in 0..width {
            let mut col = cgmath::vec3((0_f32, 0_f32, 0_f32));
            for _ in 0..samples_per_pixel {
                let du = rng.gen::<f32>();
                let u = (i as f32 + du) / (width as f32);
                let dv = rng.gen::<f32>();
                let v = (((height - j) as f32) + dv) / (height as f32);
                let ray = camera.get_ray(&mut rng, u, v);
                let p = ray.point_at_parameter(2_f32);
                col += color(ray, &world, &mut rng, 0);
            }
            col /= samples_per_pixel as f32;
            col = cgmath::vec3((f32::sqrt(col[0]), f32::sqrt(col[1]), f32::sqrt(col[2])));
            let ir = (255.99 * col[0]) as u32;
            let ig = (255.99 * col[1]) as u32;
            let ib = (255.99 * col[2]) as u32;

            data.push(to_bgra(ir, ig, ib));
        }
    }

    Image {
        width: width,
        height: height,
        data: data,
    }
}

fn write_image_to_file(image: &Image, file: &mut File) -> io::Result<()> {
    write!(file, "P3\n{} {}\n255\n", image.width, image.height).unwrap();


    Ok(())
}

fn main() -> io::Result<()> {
    let width = 320;
    let height = 240;
    let samples_per_pixel = SAMPLES_PER_PIXEL;
    let mut rng = rand::prelude::thread_rng();
    let world = generate_scene(&mut rng);
    let camera = camera(width, height);

    let image = render(width, height, samples_per_pixel, camera, world);
    let mut file = File::create("output.ppm").unwrap();
    write_image_to_file(&image, &mut file)
}
