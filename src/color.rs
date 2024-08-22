use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        f64::sqrt(linear_component)
    } else {
        0.0
    }
}

#[allow(clippy::cast_possible_truncation)]
pub fn write<T>(out: &mut T, pixel_color: &Color) -> std::io::Result<()>
where
    T: std::io::Write,
{
    static INTENSITY: Interval = Interval {
        min: 0.0,
        max: 0.999,
    };

    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let r = (256.0 * INTENSITY.clamp(r)) as i32;
    let g = (256.0 * INTENSITY.clamp(g)) as i32;
    let b = (256.0 * INTENSITY.clamp(b)) as i32;

    writeln!(out, "{r} {g} {b}")?;
    Ok(())
}
