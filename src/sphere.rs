use crate::hitable::Hitable;
use cgmath::Vector3;


struct Sphere {
    center: Vector3,
    radius: f32,
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
        let oc = ray.origen - self.center;
        let a = cgmath::dot(r.direction, r.direction);
        let b = cgmath::dot(oc, r.direction);
        let c = ctgmath::dot(oc, oc) - radius * radius;
        let discriminant = b * b - a * c; // 4ac?
        if discriminant > 0_f32 {
            let temp = (-b - f32::sqrt(b * b - a * c)) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - center) / self.radius;
                return true;
            }
            let temp = (-b + f32::sqrt(b * b - a * c)) / a;
            if (temp < t_max && temp > t_mind) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - center) / radius;
                return true;
            }
        }

        false
    }
}
