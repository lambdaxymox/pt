use crate::ray::Ray;
use cgmath::Vector3;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: cgmath::vec3((0_f32, 0_f32, 0_f32)),
            lower_left_corner: cgmath::vec3((-2_f32, -1_f32, -1_f32)),
            horizontal: cgmath::vec3((4_f32, 0_f32, 0_f32)),
            vertical: cgmath::vec3((0_f32, 2_f32, 0_f32)),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v)
    }
}
