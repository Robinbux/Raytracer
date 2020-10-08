use crate::vec3::Color;
use rand::Rng;

// Write the translated [0,255] value of each color component.
pub fn write_color(pixel_color: Color, sample_per_pixel: u16) {
    let mut r = pixel_color.r();
    let mut g = pixel_color.g();
    let mut b = pixel_color.b();

    // Divide the color by the number of samples
    let scale = 1.0 / f64::from(sample_per_pixel);
    r *= scale;
    g *= scale;
    b *= scale;

    // Write the translated [0,255] value of each color component
    println!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    )
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

// Returns a random real in [0,1).
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen();
}

// Returns a random real in [min,max).
pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min, max);
}
