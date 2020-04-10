use crate::ray::Ray;
use crate::sample;
use cgmath::{Magnitude, Vector3};


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
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HitableList {
    items: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            items: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, item: Box<dyn Hitable>) {
        self.items.push(item);
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far: f32 = t_max;
        for item in self.items.iter() {
            let mut temp_rec = HitRecord::new(
                0_f32,
                cgmath::vec3((0_f32, 0_f32, 0_f32)), 
                cgmath::vec3((0_f32, 0_f32, 0_f32)),
                Material::NullMaterial(NullMaterial::new())
            );
            if item.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}


#[derive(Copy, Clone)]
pub struct NullMaterial {}

impl NullMaterial {
    pub fn new() -> NullMaterial {
        NullMaterial {}
    }

    pub fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
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

    pub fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + sample::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;

        true
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vector3,
}

impl Metal {
    pub fn new(albedo: Vector3) -> Metal {
        Metal {
            albedo: albedo,
        }
    }

    pub fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let reflected = reflect(ray_in.direction.normalize(), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
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
    pub fn scatter(&self, ray_in: Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        match *self {
            Material::NullMaterial(null) => null.scatter(ray_in, rec, attenuation, scattered),
            Material::Metal(metal) => metal.scatter(ray_in, rec, attenuation, scattered),
            Material::Lambertian(lambertian) => lambertian.scatter(ray_in, rec, attenuation, scattered),
        }
    }

    pub fn null() -> Material {
        Material::NullMaterial(NullMaterial::new())
    }

    pub fn lambertian(albedo: Vector3) -> Material {
        Material::Lambertian(Lambertian::new(albedo))
    }
    
    pub fn metal(albedo: Vector3) -> Material {
        Material::Metal(Metal::new(albedo))
    }
}
