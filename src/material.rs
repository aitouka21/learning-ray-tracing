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
                let refracted = -Vec3::refract(&unit_direction, &hit_record.normal, ri);
                let scattered = Ray::new(hit_record.p, refracted);

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
