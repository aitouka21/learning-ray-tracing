use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

#[allow(clippy::cast_possible_truncation)]
pub fn write<T>(out: &mut T, pixel_color: &Color) -> std::io::Result<()>
where
    T: std::io::Write,
{
    static INTENSITY: Interval = Interval {
        min: 0.0,
        max: 0.999,
    };
    let r = (256.0 * INTENSITY.clamp(pixel_color.x())) as i32;
    let g = (256.0 * INTENSITY.clamp(pixel_color.y())) as i32;
    let b = (256.0 * INTENSITY.clamp(pixel_color.z())) as i32;

    writeln!(out, "{r} {g} {b}")?;
    Ok(())
}
