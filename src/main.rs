use std::fs;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

pub fn to_byte_rgb(color: f64) -> u8 {
    (255.999 * color) as u8
}

fn main() {
    let mut output = String::new();
    output += format!("P3\n{} {}\n256\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_str();
    let mut j = IMAGE_HEIGHT - 1;
    while j >= 0 {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            output +=
                format!("{} {} {}\n", to_byte_rgb(r), to_byte_rgb(g), to_byte_rgb(b)).as_str();
        }

        j -= 1;
    }

    fs::write("./test.ppm", output).expect("Failed to write file");
}
