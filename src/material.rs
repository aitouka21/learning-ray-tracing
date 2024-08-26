use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}

impl Material {
    #[allow(clippy::unnecessary_wraps)]
    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = hit_record.normal;
                }

                let result = ScatterResult {
                    scattered: Ray::new(hit_record.p, scatter_direction),
                    attenuation: *albedo,
                };
                Some(result)
            }
            Self::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(r_in.direction(), &hit_record.normal).unit()
                    + (Vec3::random_unit_vector() * fuzz);

                let scattered = Ray::new(hit_record.p, reflected);

                if Vec3::dot(scattered.direction(), &hit_record.normal) <= 0.0 {
                    return None;
                }

                let result = ScatterResult {
                    attenuation: *albedo,
                    scattered,
                };
                Some(result)
            }
            Self::Dielectric { refraction_index } => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let ri = if hit_record.front_face {
                    refraction_index.recip()
                } else {
                    *refraction_index
                };

                let unit_direction = r_in.direction().unit();
                let cos_theta = Vec3::dot(&-unit_direction, &hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = ri * sin_theta > 1.0;
                let direction = if cannot_refract || reflectance(cos_theta, ri) > rand::random() {
                    Vec3::reflect(&unit_direction, &hit_record.normal)
                } else {
                    Vec3::refract(&unit_direction, &hit_record.normal, ri)
                };
                let scattered = Ray::new(hit_record.p, direction);

                let result = ScatterResult {
                    attenuation,
                    scattered,
                };
                Some(result)
            }
        }
    }

    pub fn lambertian(albedo: Color) -> Self {
        Material::Lambertian { albedo }
    }

    pub fn metal(albedo: Color, fuzz: f64) -> Self {
        if fuzz < 1.0 {
            Material::Metal { albedo, fuzz }
        } else {
            Material::Metal { albedo, fuzz: 1.0 }
        }
    }

    pub fn dielectric(refraction_index: f64) -> Self {
        Material::Dielectric { refraction_index }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Schlick's approximation
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
