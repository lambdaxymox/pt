use rand::prelude::*;
use cgmath::{Magnitude, Vector3};


pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector3 {
    loop {
        let a = rng.gen::<f32>();
        let b = rng.gen::<f32>();
        let c = rng.gen::<f32>();
        let p = cgmath::vec3((a, b, c)) * 2_f32 - cgmath::vec3((1_f32, 1_f32, 1_f32));

        // If the sample falls inside the unit sphere, we can return.
        if p.magnitude() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vector3 {
    loop {
        let p = cgmath::vec3((
            2_f32 * rng.gen::<f32>() - 1_f32,
            2_f32 * rng.gen::<f32>() - 1_f32,
            0_f32,
        ));

        // If the sample falls inside the unit disk, we can return.
        if p.magnitude2() < 1.0 {
            return p;
        }
    }
}
