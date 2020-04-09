extern crate cgmath;

mod ray;
mod hitable;
mod hitable_list;
mod sphere;

use std::fs::File;
use std::io;
use std::io::Write;

use cgmath::{Vector3, Magnitude};
use ray::Ray;
use sphere::Sphere;
use hitable::{Hitable, HitRecord};
use hitable_list::HitableList;
use std::f32;


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
    let nx = 800;
    let ny = 400;
    write!(&mut file, "P3\n{} {}\n255\n", nx, ny).unwrap();
    let lower_left_corner = cgmath::vec3((-2.0, -1.0, -1.0));
    let horizontal = cgmath::vec3((4.0, 0.0, 0.0));
    let vertical = cgmath::vec3((0.0, 2.0, 0.0));
    let origin = cgmath::vec3((0.0, 0.0, 0.0));
    let mut world = HitableList::new();
    world.push(Box::new(Sphere::new(cgmath::vec3((0_f32, 0_f32, -1_f32)), 0.5)));
    world.push(Box::new(Sphere::new(cgmath::vec3((0_f32, -100.5, -1_f32)), 100_f32)));
    for j in 0..(ny - 1) {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = (ny - j) as f32 / ny as f32;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            
            let p = ray.point_at_parameter(2_f32);
            let col = color(ray, &world);
            let ir = (255.99 * col[0]) as u32;
            let ig = (255.99 * col[1]) as u32;
            let ib = (255.99 * col[2]) as u32;
            write!(&mut file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    Ok(())
}
