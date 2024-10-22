use std::sync::Arc;

use crate::{aabb::AABB, hittable::Hittable, hittable_list::HittableList, interval::Interval};

pub struct BVH {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bbox: AABB,
}

impl BVH {
    pub fn from_hittable(world: HittableList) -> Self {
        BVH::new(&mut world.list.clone())
    }

    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let len = objects.len();

        // Building the BBOX from the object span
        let mut bbox = AABB::EMPTY;
        for i in 0..len {
            bbox = AABB::from_bboxes(&bbox, &objects[i].bounding_box())
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => BVH::box_x_compare,
            1 => BVH::box_y_compare,
            _ => BVH::box_z_compare,
        };

        let object_span = len;

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if object_span == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if object_span == 2 {
            left = objects[0].clone();
            right = objects[1].clone();
        } else {
            objects.sort_by(|a, b| comparator(a, b).cmp(&comparator(b, a))); 

            let mid = object_span / 2;
            left = Arc::new(BVH::new(&mut objects[0..mid]));
            right = Arc::new(BVH::new(&mut objects[mid..len]));
        }

        BVH {
            left: left,
            right: right,
            bbox: bbox,
        }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> bool {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);

        let res = a_axis_interval.min < b_axis_interval.min;
        return res;
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        BVH::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        BVH::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        BVH::box_compare(a, b, 2)
    }
}

impl Hittable for BVH {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &mut crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        // Exit early if bbox of this node is not hit
        if !self.bbox.hit(r, *ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(r, &mut Interval::new(ray_t.min, if hit_left {rec.t} else {ray_t.max}), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
