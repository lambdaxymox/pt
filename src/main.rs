extern crate cgmath;
extern crate rand;

mod ray;
mod hitable;
mod hitable_list;
mod sphere;
mod camera;

use rand::prelude::*;

use std::fs::File;
use std::io;
use std::io::Write;
use std::f32;

use cgmath::{Vector3, Magnitude};
use camera::Camera;
use ray::Ray;
use sphere::Sphere;
use hitable::{Hitable, HitRecord};
use hitable_list::HitableList;



fn color<H: Hitable>(ray: Ray, world: &H) -> Vector3 {
    let mut rec = HitRecord::new(
        0_f32,
        cgmath::vec3((0_f32, 0_f32, 0_f32)), 
        cgmath::vec3((0_f32, 0_f32, 0_f32)),
    );
    if world.hit(ray, 0_f32, f32::MAX, &mut rec) {
        return cgmath::vec3((rec.normal.x + 1_f32, rec.normal.y + 1_f32, rec.normal.z + 1_f32)) * 0.5;
    } else {
        let unit_direction = ray.direction.normalize();
        let t = (unit_direction.y + 1_f32) * 0.5;
        return cgmath::vec3((1.0, 1.0, 1.0)) * (1_f32 - t) + cgmath::vec3((0.5, 0.7, 1.0)) * t;
    }

}

fn main() -> io::Result<()> {
    let mut file = File::create("output.ppm")?;
    let mut rng = rand::prelude::thread_rng();
    let nx = 200;
    let ny = 100;
    let ns = 100;
    write!(&mut file, "P3\n{} {}\n255\n", nx, ny).unwrap();
    let mut world = HitableList::new();
    world.push(Box::new(Sphere::new(cgmath::vec3((0_f32, 0_f32, -1_f32)), 0.5)));
    world.push(Box::new(Sphere::new(cgmath::vec3((0_f32, -100.5, -1_f32)), 100_f32)));
    let camera = Camera::new();
    for j in 0..(ny - 1) {
        for i in 0..nx {
            let mut col = cgmath::vec3((0_f32, 0_f32, 0_f32));
            for s in 0..ns {
                let du: f32 = rng.gen();
                let u = (i as f32 + du) / (nx as f32);
                let dv: f32 = rng.gen();
                let v = (((ny - j) as f32) + dv) / (ny as f32);
                let ray = camera.get_ray(u, v);
                let p = ray.point_at_parameter(2_f32);
                col += color(ray, &world);
            }
            col /= ns as f32;
            let ir = (255.99 * col[0]) as u32;
            let ig = (255.99 * col[1]) as u32;
            let ib = (255.99 * col[2]) as u32;
            write!(&mut file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    Ok(())
}
