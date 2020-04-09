use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use cgmath::Vector3;


pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = cgmath::dot(ray.direction, ray.direction);
        let b = cgmath::dot(oc, ray.direction);
        let c = cgmath::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c; // 4ac?
        if discriminant > 0_f32 {
            let temp = (-b - f32::sqrt(b * b - a * c)) / a; //4ac?
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            let temp = (-b + f32::sqrt(b * b - a * c)) / a; // 4ac?
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }

        false
    }
}
