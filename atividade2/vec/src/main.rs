use std::io::Write;

use vec::{Vec3, write_color};
pub fn main() {
    let img_width: u32 = 256;
    let img_height: u32 = 256;
    println!("P3\n{} {}\n255", img_width, img_height);

    for j in 0..img_height {
        write!(std::io::stderr(), "\rScanlines remaining: {} ", img_height - j).unwrap();
        for i in 0..img_width {
            let pixel_color = Vec3{coords: vec![i as f64/(img_width as f64-1.0),j as f64/(img_width-1)as f64,0.0]};
            write_color(pixel_color);
        }
    }
}