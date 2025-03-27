// Copyright (c) Anderson Karl <andersonlkarl@gmail.com>. Licensed under the MIT Licence.
// See the LICENCE file in the repository root for full licence text.

use dicom::object;
use dicom_pixeldata::{BitDepthOption, ConvertOptions, PixelDecoder};
use std::time::Instant;
use std::fs;
use std::error::Error;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::io::{Cursor, Read};
use image::{DynamicImage, ImageFormat, imageops::FilterType, ImageBuffer};

fn generate_dicom_thumbnail(dicom_base64: &str) -> Result<String, Box<dyn Error>> {
    // Decode base64 to bytes
    let dicom_bytes = BASE64.decode(dicom_base64)?;
    
    // Create a cursor to read the bytes as a file
    let cursor = Cursor::new(dicom_bytes);
    
    // Open DICOM from memory
    let obj = object::from_reader(cursor)?;
    let image = match obj.decode_pixel_data() {
        Ok(img) => img,
        Err(e) => {
            println!("Error decoding pixel data: {}", e);
            return Err(Box::new(e));
        }
    };

    let options = ConvertOptions::new().with_bit_depth(BitDepthOption::Auto);

    let img = match image.to_dynamic_image_with_options(0, &options) {
        Ok(img) => {
            let width = img.width() as u32;
            let height = img.height() as u32;
            let pixels = img.to_rgb8();
            ImageBuffer::from_raw(width, height, pixels.into_raw())
                .map(DynamicImage::ImageRgb8)
                .ok_or("Failed to create image buffer")?
        },
        Err(e) => {
            println!("Error converting image: {}", e);
            return Err(Box::new(e));
        }
    };

    let thumbnail = img.resize(150, 150, FilterType::Lanczos3);

    // Create a buffer to store the PNG data
    let mut png_buffer = Vec::new();
    thumbnail.write_to(&mut Cursor::new(&mut png_buffer), ImageFormat::Png)?;

    // Convert the PNG buffer to base64
    let thumbnail_base64 = BASE64.encode(png_buffer);

    Ok(thumbnail_base64)
}

fn main() -> Result<(), Box<dyn Error>> {
    const DICOM_DIR_FILE: &str = "image-000001.dcm";
    
    // Read the DICOM file into memory
    let mut file = fs::File::open(DICOM_DIR_FILE)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Convert the file content to base64
    let dicom_base64 = BASE64.encode(&buffer);

    // Process the base64 DICOM data
    let start_time = Instant::now();
    match generate_dicom_thumbnail(&dicom_base64) {
        Ok(thumbnail_base64) => {
            // Decode the base64 thumbnail back to PNG bytes
            let png_bytes = BASE64.decode(thumbnail_base64)?;
            
            // Save the PNG bytes to a file
            fs::write("output_thumbnail.png", png_bytes)?;
            
            let elapsed_time = start_time.elapsed();
            println!("Successfully generated thumbnail. Took: {:.2?}", elapsed_time);
        },
        Err(e) => {
            println!("Error generating thumbnail: {}", e);
        }
    }

    Ok(())
}
