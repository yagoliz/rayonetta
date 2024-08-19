use crate::{color::Color, hittable::HitRecord, ray::Ray, utils::random_uniform, vec3::{dot, random_unit_sphere, reflect, refract, unit_vector, Vec3}};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

// Diffuse material
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

// Metallic material
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Metal { albedo: color, fuzz: if fuzz < 1.0 {fuzz} else {1.0} }
    }
}

impl Material for Metal {
    fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
        
        let mut reflected = reflect(r_in.direction(), rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_sphere());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return dot(scattered.direction(), rec.normal) > 0.0
    }
}

// Dielectric
pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index: refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlick's approximation
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0-r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {1.0/self.refraction_index} else {self.refraction_index};

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;

        let direction: Vec3;
        if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_uniform() {
            direction = reflect(unit_direction, rec.normal);
        } else {
            direction = refract(unit_direction, rec.normal, ri);
        }

        *scattered = Ray::new(rec.p, direction);
        return true;
    }
}