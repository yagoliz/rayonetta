use std::sync::Arc;

use crate::{aabb::AABB, hittable::Hittable, hittable_list::HittableList, utils::random_int};

pub struct BVH {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bbox: AABB,
}

impl BVH {
    pub fn from_hittable(world: HittableList) -> Self {
        BVH::new(&mut world.list.clone(), 0, world.list.len())
    }

    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let axis = random_int(0, 2);

        let comparator = match axis {
            0 => BVH::box_x_compare,
            1 => BVH::box_y_compare,
            _ => BVH::box_z_compare,
        };

        let object_span = end - start;

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            objects[start..end].sort_by(|a, b| {
                if comparator(a.clone(), b.clone()) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });

            let mid = start + object_span / 2;
            left = Arc::new(BVH::new(&mut objects.clone(), start, mid));
            right = Arc::new(BVH::new(&mut objects.clone(), mid, end));
        }

        let bbox = AABB::from_bboxes(&left.bounding_box(), &right.bounding_box());
        BVH {
            left: left,
            right: right,
            bbox: bbox,
        }
    }

    fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis_index: usize) -> bool {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);

        a_axis_interval.min < b_axis_interval.min
    }

    fn box_x_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
        BVH::box_compare(a, b, 0)
    }

    fn box_y_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
        BVH::box_compare(a, b, 1)
    }

    fn box_z_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
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
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(r, ray_t, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
