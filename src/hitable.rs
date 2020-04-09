use crate::ray::Ray;
use cgmath::Vector3;


#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
}

impl HitRecord {
    pub fn new(t: f32, p: Vector3, normal: Vector3) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
