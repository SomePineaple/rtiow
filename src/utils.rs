use cgmath::{InnerSpace, Vector3, vec3, dot};

pub fn to_byte_rgb(color: f64) -> u8 {
    (255.999 * color) as u8
}

pub fn color_str(color: Vector3<f64>) -> String {
    return format!(
        "{} {} {}\n",
        to_byte_rgb(color.x),
        to_byte_rgb(color.y),
        to_byte_rgb(color.z)
    );
}
