// Import the image crate
use image::{ImageFormat, RgbaImage, RgbImage};

/// This function is used to open an image and convert it to rgba8, then return it.
/// 
/// ## Return
/// It returns a Result with an Ok(RgbaImage) or a ImageError.
/// 
/// ## Implementation
/// This function utilizes image crate's image::open function to actually open the file.
/// 
/// ## Example:
/// ```
/// let img = open_image(path_to_open);
/// ```
pub fn open_image(path: &str) -> Result<RgbaImage, image::ImageError> {
    // Use the image crate's open function to open the image
    let img = image::open(path)?;
    // Convert the image to RGBA
    let rgba = img.into_rgba8();
    Ok(rgba)
}

/// This function is used to save a RgbaImage in a certain path and a certain format defined in a string.
/// 
/// ## Return
/// It returns a Result with an empty Ok or a ImageError.
/// 
/// ## Implementation
/// This function utilizes image crate's save_with_format function to actually create the file.
/// 
/// ## Example:
/// ```
/// let img = open_image(path_to_open);
/// save_img(image_to_save, path_to_save, ImageFormat::Pnm);
/// ```
pub fn save_img(img: &RgbImage, path: &str, format: ImageFormat) -> Result<(), image::ImageError> {
    // Use the image crate's save_with_format function to save the image
    img.save_with_format(path, format)
}

/// This function is used to save a RgbaImage in a certain path in Png format. 
/// 
/// ## Return
/// It returns a Result with an empty Ok or a ImageError.
/// 
/// ## Implementation
/// This function utilizes image crate's save_with_format function to actually create the file.
/// 
/// ## Example:
/// ```
/// let img = open_image(path_to_open);
/// save_png(image_to_save, path_to_save);
/// ```
pub fn save_png(img: &RgbImage, path: &str) -> Result<(), image::ImageError> {
    img.save_with_format(path, ImageFormat::Png)
}

#[test]
fn it_works() {
    //let result = add(2, 2);
    //assert_eq!(result, 4);
}
