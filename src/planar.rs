use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable_list::HittableList;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{cross, dot, unit_vector, Point3, Vec3};

pub struct Quadrilateral {
    q: Point3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    offset: f64,
    w: Vec3,
    bbox: AABB,
    mat: Arc<dyn Material + Sync + Send>,
}

impl Quadrilateral {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material + Sync + Send>) -> Self {
        let bbox_diagonal1 = AABB::from_points(q, q + u + v);
        let bbox_diagonal2 = AABB::from_points(q + u, q + v);

        let n = cross(u,v);
        let normal = unit_vector(n);
        let offset = -dot(normal, q);

        Quadrilateral {
            q: q,
            u: u,
            v: v,
            normal: normal,
            offset: offset,
            w: n / dot(n, n),
            bbox: AABB::from_bboxes(&bbox_diagonal1, &bbox_diagonal2),
            mat: mat,
        }
    }

    fn is_interior(&self, alpha: f64, beta: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return false;
        }

        rec.u = alpha;
        rec.v = beta;

        true
    }
}

impl Hittable for Quadrilateral {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let denominator = dot(self.normal, r.direction());
        if f64::abs(denominator) < 1e-6 {
            return false;
        }

        let numerator = dot(self.normal, r.origin()) + self.offset;
        let t = - numerator / denominator;

        if !ray_t.surrounds(t) {
            return false;
        }

        let intersection = r.at(t);

        let p = intersection - self.q;
        let alpha = dot(self.w, cross(p, self.v));
        let beta = dot(self.w, cross(self.u, p));

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.set_face_normal(r, &self.normal);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub fn create_box(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Arc::new(Quadrilateral::new(Point3::new(min.x(), min.y(), max.z()), dx, dy, mat.clone()))); // front
    sides.add(Arc::new(Quadrilateral::new(Point3::new(max.x(), min.y(), max.z()),-dz, dy, mat.clone()))); // right
    sides.add(Arc::new(Quadrilateral::new(Point3::new(max.x(), min.y(), min.z()),-dx, dy, mat.clone()))); // back
    sides.add(Arc::new(Quadrilateral::new(Point3::new(min.x(), min.y(), min.z()), dz, dy, mat.clone()))); // left
    sides.add(Arc::new(Quadrilateral::new(Point3::new(min.x(), max.y(), max.z()), dx,-dz, mat.clone()))); // top
    sides.add(Arc::new(Quadrilateral::new(Point3::new(min.x(), min.y(), min.z()), dx, dz, mat.clone()))); // bottom

    Arc::new(sides)
}
