use crate::convert::to_ascii;
use image::flat::Error;
use rand::prelude::*;
use std::path::Path;

#[test]
fn convert_local_images() {
    let palette = String::from(" .:-=+*#%@");
    to_ascii("imgs/apple01.jpg", &palette, true).unwrap();
}

// fn generate(path: &str, image_size: (u32, u32)) -> Result<(), Error> {
//     // This function generates an image for testing
//     let mut rng = thread_rng();
//
//     // if image exist then skip
//     if Path::new(path).exists() == true {
//         return Ok(());
//     }
//     // Create a new ImgBuf with width: imgx and height: imgy
//     let mut imgbuf = image::ImageBuffer::new(image_size.0, image_size.1);
//
//     // A redundant loop to demonstrate reading image data
//     for x in 0..image_size.0 {
//         for y in 0..image_size.1 {
//             let pixel = imgbuf.get_pixel_mut(x, y);
//             let image::Rgb(data) = *pixel;
//             *pixel = image::Rgb([data[0], rng.gen_range(0..=255) as u8, data[2]]);
//         }
//     }
//
//     // Save the image as “fractal.png”, the format is deduced from the path
//     imgbuf.save(path).unwrap();
//
//     Ok(())
// }
