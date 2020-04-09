use rand::prelude::*;
use cgmath::Vector3;


pub fn random_in_unit_sphere() -> Vector3 {
    let mut rng = thread_rng();
    loop {
        let a: f32 = rng.gen();
        let b: f32 = rng.gen();
        let c: f32 = rng.gen();
        let p = cgmath::vec3((a, b, c)) * 2_f32;

        // If the sample falls inside the unit sphere, we can exit.
        if cgmath::dot(p, p) < 1.0 {
            return p;
        }
    }
}
