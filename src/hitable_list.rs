use crate::ray::Ray;
use crate::hitable::{HitRecord, Hitable};
use cgmath::Vector3;


struct HitableList {
    items: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            items: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(
            0_f32,
            cgmath::vec3((0_f32, 0_f32, 0_f32)), 
            cgmath::vec3((0_f32, 0_f32, 0_f32)),
        );
        let mut hit_anything = false;
        let mut closest_so_far: f32 = t_max;
        for item in self.items.iter() {
            if item.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
