use crate::ray::Ray;
use crate::sample;
use crate::material::Material;
use cgmath::{Magnitude, Vector3};



pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

impl HitRecord {
    pub fn new(t: f32, p: Vector3, normal: Vector3, material: Material) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: material,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
