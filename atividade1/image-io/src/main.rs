use image_io::save_png;
pub fn main() {
    let img_width: u32 = 256;
    let img_height: u32 = 256;
    
    let mut imgbuf = image::ImageBuffer::new(img_width,img_height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        println!("x: {}", x);
        println!("y: {}", y);
        let r = (x as f32) as u8;
        let g = (y as f32) as u8;
        println!("r: {}", r);
        println!("g: {}", g);
        *pixel = image::Rgb([r, g, 0]);
    }

    let result = save_png(&imgbuf, "./gradient.png");
    let _result_image = match result {
        Ok(_) => println!("Image Saved Successfully"),
        Err(_image_error) => panic!("Problem saving the image")
    };
}