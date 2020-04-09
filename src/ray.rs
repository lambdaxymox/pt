use crate::cgmath::Vector3;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}
