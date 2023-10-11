use image_io::save_png;
use std::io::Write;
pub fn main() {
    let img_width: u32 = 256;
    let img_height: u32 = 256;
    
    let mut imgbuf = image::ImageBuffer::new(img_width,img_height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        write!(std::io::stderr(), "\rScanlines remaining: {} ", img_height - y).unwrap();
        let r = (x as f32) as u8;
        let g = (y as f32) as u8;
        *pixel = image::Rgb([r, g, 0]);
    }
    println!();

    let result = save_png(&imgbuf, "./gradient.png");
    let _result_image = match result {
        Ok(_) => println!("Image Saved Successfully"),
        Err(_image_error) => panic!("Problem saving the image")
    };

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        write!(std::io::stderr(), "\rScanlines remaining: {} ", img_height - y).unwrap();
        if 230 > x && x > 26 && 230 > y && y > 26 {
            *pixel = image::Rgb([255, 255, 255]);
        }
    }
    println!();

    let result = save_png(&imgbuf, "./rectangle.png");
    let _result_image = match result {
        Ok(_) => println!("Image Saved Successfully"),
        Err(_image_error) => panic!("Problem saving the image")
    };

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        write!(std::io::stderr(), "\rScanlines remaining: {} ", img_height - y).unwrap();
        if 200 > x && x > 56 && 200 > y && y > 56 {
            *pixel = image::Rgb([45, 60, 70]);
        }
    }
    println!();

    let result = save_png(&imgbuf, "./shape.png");
    let _result_image = match result {
        Ok(_) => println!("Image Saved Successfully"),
        Err(_image_error) => panic!("Problem saving the image")
    };

}