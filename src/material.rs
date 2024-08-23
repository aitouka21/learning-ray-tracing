use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
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
            Self::Metal { albedo } => {
                let reflected = Vec3::reflect(r_in.direction(), &hit_record.normal);

                let result = ScatterResult {
                    scattered: Ray::new(hit_record.p, reflected),
                    attenuation: *albedo,
                };
                Some(result)
            }
        }
    }

    pub fn lambertian(albedo: Color) -> Self {
        Material::Lambertian { albedo }
    }

    pub fn metal(albedo: Color) -> Self {
        Material::Metal { albedo }
    }
}
