use crate::ray::Ray;
use cgmath::Vector3;


struct HitRecord {
    t: f32,
    p: Vector3,
    normal: Vector3,
}

trait Hitable {
    fn hit(ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
