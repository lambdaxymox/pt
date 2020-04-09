extern crate cgmath;

mod ray;
mod hitable;

use std::fs::File;
use std::io;
use std::io::Write;

use cgmath::{Vector3, Magnitude};
use ray::Ray;



fn hit_sphere(center: Vector3, radius: f32, ray: Ray) -> f32 {
    let oc = ray.origin - center;
    let a = cgmath::dot(ray.direction, ray.direction);
    let b = 2_f32 * cgmath::dot(oc, ray.direction);
    let c = cgmath::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4_f32 * a * c;
    if discriminant < 0_f32 {
        return -1_f32;
    } else {
        return (-b - f32::sqrt(discriminant)) / (2_f32 * a);
    }
}

fn color(ray: Ray) -> Vector3 {
    let sphere_center = cgmath::vec3((0.0, 0.0, -1.0));
    let sphere_radius = 0.5;
    let t = hit_sphere(sphere_center, sphere_radius, ray);
    if t > 0_f32 {
        let N = (ray.point_at_parameter(t) - sphere_center).normalize();
        return cgmath::vec3((N.x + 1_f32, N.y + 1_f32, N.z + 1_f32)) * 0.5;
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    
    cgmath::vec3((1.0, 1.0, 1.0)) * (1.0 - t) + cgmath::vec3((0.5, 0.7, 1.0)) * t
}

fn main() -> io::Result<()> {
    let mut file = File::create("output.ppm")?;
    let nx = 200;
    let ny = 100;
    write!(&mut file, "P3\n{} {}\n255\n", nx, ny).unwrap();
    let lower_left_corner = cgmath::vec3((-2.0, -1.0, -1.0));
    let horizontal = cgmath::vec3((4.0, 0.0, 0.0));
    let vertical = cgmath::vec3((0.0, 2.0, 0.0));
    let origin = cgmath::vec3((0.0, 0.0, 0.0));
    for j in 0..(ny - 1) {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = (ny - j) as f32 / ny as f32;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(ray);
            let ir = (255.99 * col[0]) as u32;
            let ig = (255.99 * col[1]) as u32;
            let ib = (255.99 * col[2]) as u32;
            write!(&mut file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    Ok(())
}
