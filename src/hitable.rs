use crate::ray::Ray;
use crate::sample;
use cgmath::{Magnitude, Vector3};


fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - n * 2_f32 * cgmath::dot(v, n)
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    pub fn new(t: f32, p: Vector3, normal: Vector3, material: Box<dyn Material>) -> HitRecord {
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

pub trait Material {
    fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool;
}

pub struct NullMaterial {}

impl NullMaterial {
    pub fn new() -> NullMaterial {
        NullMaterial {}
    }
}

impl Material for NullMaterial {
    fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian {
            albedo: albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + sample::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Vector3,
}

impl Metal {
    pub fn new(albedo: Vector3) -> Metal {
        Metal {
            albedo: albedo,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let reflected = reflect(ray_in.direction.normalize(), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        cgmath::dot(scattered.direction, rec.normal) > 0_f32
    }
}
