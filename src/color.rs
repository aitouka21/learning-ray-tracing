use crate::vec3::Vec3;

#[allow(unused)]
pub type Color = Vec3;

#[allow(unused)]
pub fn write<T>(out: &mut T, pixel_color: &Color) -> std::io::Result<()>
where
    T: std::io::Write,
{
    let r = (pixel_color.x() * 255.999) as i32;
    let g = (pixel_color.y() * 255.999) as i32;
    let b = (pixel_color.z() * 255.999) as i32;

    writeln!(out, "{r} {g} {b}")?;
    Ok(())
}
