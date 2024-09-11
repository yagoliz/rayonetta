// Testing
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use color::Color;
    use hittable::{HitRecord, Hittable};
    use interval::Interval;
    use material::Lambertian;
    use ray::Ray;
    use rayonetta::*;
    use sphere::Sphere;
    use utils::INFINITY;
    use vec3::{Point3, Vec3};

    #[test]
    fn sphere_intersect_simple() {
        let m = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 1.0), 0.5, m.clone());

        let r = Ray::new(Point3::empty(), Vec3::new(0.0, 0.0, 1.0));
        let mut ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::new();
        sphere.hit(&r, &mut ray_t, &mut rec);

        assert!((rec.p - Point3::new(0.0, 0.0, 0.5)).length() < 1e-6);
        assert!(rec.t - 0.5 < 1e-8);
    }

    #[test]
    fn sphere_intersect_angle() {
        let m = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 1.0), 0.5, m.clone());

        let r = Ray::new(Point3::empty(), Vec3::new(-0.2, 0.0, 1.0));
        let mut ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::new();
        sphere.hit(&r, &mut ray_t, &mut rec);

        let t = 0.5105369462;
        assert!((rec.p - Point3::new(-0.2 * t, 0.0, t)).length() < 1e-6);
        assert!(rec.t - t < 1e-8);
    }

    #[test]
    fn sphere_intersect_tangent() {
        let m = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 1.0), 0.5, m.clone());

        let r = Ray::new(Point3::new(-0.5, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let mut ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::new();

        assert!(sphere.hit(&r, &mut ray_t, &mut rec));
        assert!((rec.p - Point3::new(-0.5, 0.0, 1.0)).length() < 1e-6);
        assert_eq!(rec.t, 1.0);
    }

    #[test]
    fn sphere_no_intersect() {
        let m = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 1.0), 0.5, m.clone());

        let r = Ray::new(Point3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let mut ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::new();

        assert!(!sphere.hit(&r, &mut ray_t, &mut rec));
    }

    #[test]
    fn sphere_no_intersect_interval() {
        let m = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 1.0), 0.5, m.clone());

        let r = Ray::new(Point3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let mut ray_t = Interval::new(0.001, 0.1);
        let mut rec = HitRecord::new();

        assert!(!sphere.hit(&r, &mut ray_t, &mut rec));
    }
}