use std::sync::Arc;

use crate::aabb::AABB;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub type HittableObject = Arc<dyn Hittable>;

pub struct HittableList {
    pub list: Vec<HittableObject>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { list: Vec::new(), bbox: AABB::EMPTY }
    }

    pub fn from_object(obj: HittableObject) -> Self {
        let mut list = HittableList::new();
        list.add(obj);
        list
    }

    pub fn add(&mut self, obj: HittableObject) {
        self.bbox = AABB::from_bboxes(&self.bbox, &obj.bounding_box());
        self.list.push(obj);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest = ray_t.max;

        for object in self.list.iter() {
            let mut temp = HitRecord::new();
            if object.hit(r, &mut Interval::new(ray_t.min, closest), &mut temp) {
                hit_anything = true;
                closest = temp.t;

                *rec = temp;
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}