use crate::camera::Camera;
use crate::ray::Ray;
use crate::hitable_list::HitableList;
use crate::material::*;
use cgmath::{Vector3, Magnitude};
use rand::prelude::*;


const MAX_DEPTH: u32 = 16;


#[inline]
fn component_multiply(v1: Vector3, v2: Vector3) -> Vector3 {
    cgmath::vec3((v1.x * v2.x, v1.y * v2.y, v1.z * v2.z))
}

fn color<H: Hitable>(ray: Ray, world: &H, rng: &mut ThreadRng, depth: u32) -> Vector3 {
    match world.hit(&ray, 0.001, std::f32::MAX) {
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

pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgba {
    #[inline]
    fn new(r: u8, g: u8, b: u8) -> Rgba {
        Rgba { 
            r: r,
            b: b,
            g: g,
        }
    }
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Rgba>,
}

pub fn render(width: u32, height: u32, samples_per_pixel: u32, camera: Camera, world: HitableList) -> Image {
    let mut rng = rand::prelude::thread_rng();
    let mut data = vec![];
    for j in 0..height {
        println!("Row {}.", j);
        for i in 0..width {
            let mut col = cgmath::vec3((0_f32, 0_f32, 0_f32));
            for _ in 0..samples_per_pixel {
                let du = rng.gen::<f32>();
                let u = (i as f32 + du) / (width as f32);
                let dv = rng.gen::<f32>();
                let v = (((height - j) as f32) + dv) / (height as f32);
                let ray = camera.get_ray(&mut rng, u, v);
                col += color(ray, &world, &mut rng, 0);
            }
            col /= samples_per_pixel as f32;
            col = cgmath::vec3((f32::sqrt(col[0]), f32::sqrt(col[1]), f32::sqrt(col[2])));
            let ir = (255.99 * col[0]) as u8;
            let ig = (255.99 * col[1]) as u8;
            let ib = (255.99 * col[2]) as u8;

            data.push(Rgba::new(ir, ig, ib));
        }
    }

    Image {
        width: width,
        height: height,
        data: data,
    }
}
