use crate::ray::Ray;
use crate::hitable::HitRecord;
use cgmath::Vector3;


pub trait Material {
    fn scatter(ray_in: Ray, rec: &mut HitRecord, attenuation: Vector3, scatters: Ray) -> bool;
}
