use crate::ray::Ray;
use crate::sample;
use cgmath::Vector3;
use std::rc::Rc;



#[derive(Copy, Clone)]
pub struct Material {}


#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: Rc<Material>,
}

impl HitRecord {
    pub fn new(t: f32, p: Vector3, normal: Vector3) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: Rc::new(Material {}),
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub trait HitableMaterial {
    fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool;
}

struct Lambertian {
    albedo: Vector3,
}

impl HitableMaterial for Lambertian {
    fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + sample::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;

        true
    }
}
