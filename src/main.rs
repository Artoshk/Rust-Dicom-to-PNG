// Copyright (c) Anderson Karl <andersonlkarl@gmail.com>. Licensed under the MIT Licence.
// See the LICENCE file in the repository root for full licence text.

use dicom::object::open_file;
use dicom_pixeldata::{image, BitDepthOption, ConvertOptions, PixelDecoder};
use std::time::Instant;
use std::fs;

fn get_all_dicom_files_recursively(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(get_all_dicom_files_recursively(&path.to_string_lossy())?);
        } else if path.is_file() && path.to_string_lossy().ends_with(".dcm") {
            files.push(path.to_string_lossy().to_string());
        }
    }
    Ok(files)
}

fn generate_dicom_thumbnail(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = path.split('/').last().unwrap()
        .replace(".dcm", ".png");
    let output_path = format!("thumbnails/{}", file_name);
    
    let obj = open_file(path)?;
    let image = match obj.decode_pixel_data() {
        Ok(img) => img,
        Err(e) => {
            println!("Error decoding pixel data for {}: {}", path, e);
            return Ok(());  // Skip this file but continue processing others
        }
    };

    let options = ConvertOptions::new().with_bit_depth(BitDepthOption::Auto);
    
    let dynamic_image = match image.to_dynamic_image_with_options(0, &options) {
        Ok(img) => img,
        Err(e) => {
            println!("Error converting image for {}: {}", path, e);
            return Ok(());  // Skip this file but continue processing others
        }
    };
    
    let thumbnail = dynamic_image.resize(150, 150, image::imageops::FilterType::Lanczos3);
    
    fs::create_dir_all("thumbnails")?;
    
    if let Err(e) = thumbnail.save(&output_path) {
        println!("Error saving thumbnail for {}: {}", path, e);
        return Ok(());  // Skip this file but continue processing others
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Your folder that contains the dicom files
    const DICOM_DIR: &str = "/mnt/d/FilterDicom";
    let files = get_all_dicom_files_recursively(DICOM_DIR)?;
    let mut success_count = 0;
    let total_files = files.len();
    
    for file in files {
        println!("Processing file: {}", file);
        let start_time = Instant::now();
        if generate_dicom_thumbnail(&file).is_ok() {
            success_count += 1;
        }
        let elapsed_time = start_time.elapsed();
        println!("Processing time: {:.2?}", elapsed_time);
    }
    
    println!("Successfully processed {}/{} files", success_count, total_files);
    Ok(())
}
