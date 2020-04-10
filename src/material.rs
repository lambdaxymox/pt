use crate::ray::Ray;
use crate::sample;
use cgmath::{Magnitude, Vector3};
use rand::prelude::*;


#[inline]
fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - n * 2_f32 * cgmath::dot(v, n)
}

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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}


#[derive(Copy, Clone)]
pub struct NullMaterial {}

impl NullMaterial {
    pub fn new() -> NullMaterial {
        NullMaterial {}
    }

    pub fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray, rng: &mut ThreadRng) -> bool {
        false
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian {
            albedo: albedo,
        }
    }

    pub fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray, rng: &mut ThreadRng) -> bool {
        let target = rec.p + rec.normal + sample::random_in_unit_sphere(rng);
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;

        true
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vector3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f32) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz,
        }
    }

    pub fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray, rng: &mut ThreadRng) -> bool {
        let reflected = reflect(ray_in.direction.normalize(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + sample::random_in_unit_sphere(rng) * self.fuzz);
        *attenuation = self.albedo;

        cgmath::dot(scattered.direction, rec.normal) > 0_f32
    }
}


#[derive(Copy, Clone)]
pub enum Material {
    NullMaterial(NullMaterial),
    Metal(Metal),
    Lambertian(Lambertian),
}

impl Material {
    pub fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray, rng: &mut ThreadRng) -> bool {
        match *self {
            Material::NullMaterial(null) => null.scatter(ray_in, rec, attenuation, scattered, rng),
            Material::Metal(metal) => metal.scatter(ray_in, rec, attenuation, scattered, rng),
            Material::Lambertian(lambertian) => lambertian.scatter(ray_in, rec, attenuation, scattered, rng),
        }
    }

    pub fn null() -> Material {
        Material::NullMaterial(NullMaterial::new())
    }

    pub fn lambertian(albedo: Vector3) -> Material {
        Material::Lambertian(Lambertian::new(albedo))
    }
    
    pub fn metal(albedo: Vector3, fuzz: f32) -> Material {
        Material::Metal(Metal::new(albedo, fuzz))
    }
}
