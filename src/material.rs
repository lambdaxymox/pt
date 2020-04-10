use crate::ray::Ray;
use crate::sample;
use cgmath::{Magnitude, Vector3};
use rand::prelude::*;


#[inline]
fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - n * 2_f32 * cgmath::dot(v, n)
}

#[derive(Copy, Clone)]
pub struct Scatter {
    pub attenuation: Vector3,
    pub ray: Ray,
}

impl Scatter {
    pub fn new(attenuation: Vector3, ray: Ray) -> Scatter {
        Scatter { 
            attenuation: attenuation, 
            ray: ray,
        }
    }
}

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vector3, normal: Vector3, material: &'a Material) -> HitRecord<'a> {
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
pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian {
            albedo: albedo,
        }
    }

    pub fn scatter(&self, ray_in: Ray, hit: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let target = hit.p + hit.normal + sample::random_in_unit_sphere(rng);
        let attenuation = self.albedo;
        let scattered = Ray::new(hit.p, target - hit.p);

        Scatter::new(attenuation, scattered)
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

    pub fn scatter(&self, ray_in: Ray, hit: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let reflected = reflect(ray_in.direction.normalize(), hit.normal);
        let attenuation = self.albedo;
        let scattered = Ray::new(hit.p, reflected + sample::random_in_unit_sphere(rng) * self.fuzz);
        
        Scatter::new(attenuation, scattered)
    }
}


#[derive(Copy, Clone)]
pub enum Material {
    Metal(Metal),
    Lambertian(Lambertian),
}

impl Material {
    pub fn scatter(&self, ray_in: Ray, hit: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        match *self {
            Material::Metal(metal) => metal.scatter(ray_in, hit, rng),
            Material::Lambertian(lambertian) => lambertian.scatter(ray_in, hit, rng),
        }
    }

    pub fn lambertian(albedo: Vector3) -> Material {
        Material::Lambertian(Lambertian::new(albedo))
    }
    
    pub fn metal(albedo: Vector3, fuzz: f32) -> Material {
        Material::Metal(Metal::new(albedo, fuzz))
    }
}
