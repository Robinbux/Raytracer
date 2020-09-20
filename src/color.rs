use crate::vec3::Color;

// Write the translated [0,255] value of each color component.
pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (55.999 * pixel_color.z()) as i32
    )
}
