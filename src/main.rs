// Copyright (c) Anderson Karl <andersonlkarl@gmail.com>. Licensed under the MIT Licence.
// See the LICENCE file in the repository root for full licence text.

use dicom::object::open_file;
use dicom_pixeldata::{image, BitDepthOption, ConvertOptions, PixelDecoder};
use std::time::Instant;
use std::fs;
use std::path::Path;
use std::error::Error;
use rayon::prelude::*;
use sha2::{Sha256, Digest};

fn get_all_dicom_files_recursively(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(get_all_dicom_files_recursively(&path)?);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("dcm") {
            files.push(path.to_string_lossy().to_string());
        }
    }
    Ok(files)
}

fn generate_dicom_thumbnail(path: &str) -> Result<(), Box<dyn Error>> {
    // Compute the hash from the file path
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    // Extract the original file name
    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");

    // Combine hash and file name, replacing the extension with .png
    let new_file_name = format!("{}_{}", hash, file_name.replace(".dcm", ".png"));

    // Construct the output path
    let output_path = Path::new("thumbnails").join(new_file_name);

    let obj = open_file(path)?;
    let image = match obj.decode_pixel_data() {
        Ok(img) => img,
        Err(e) => {
            println!("Error decoding pixel data for {}: {}", path, e);
            return Err(Box::new(e)); // Skip this file but continue processing others
        }
    };

    let options = ConvertOptions::new().with_bit_depth(BitDepthOption::Auto);

    {
        let dynamic_image = match image.to_dynamic_image_with_options(0, &options) {
            Ok(img) => img,
            Err(e) => {
                println!("Error converting image for {}: {}", path, e);
                return Err(Box::new(e)); // Skip this file but continue processing others
            }
        };

        let thumbnail = dynamic_image.resize(150, 150, image::imageops::FilterType::Lanczos3);

        fs::create_dir_all("thumbnails")?;
        if let Err(e) = thumbnail.save(&output_path) {
            println!("Error saving thumbnail for {}: {}", path, e);
            return Err(Box::new(e)); // Skip this file but continue processing others
        }
    } // Ensure `dynamic_image` and `thumbnail` are dropped here

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Change this to the path to your DICOM files
    const DICOM_DIR: &str = "/mnt/d/FilterDicom";
    let files = get_all_dicom_files_recursively(Path::new(DICOM_DIR))?;
    let total_files = files.len();

    let success_count = files.par_iter().map(|file| {
        let start_time = Instant::now();

        let result = generate_dicom_thumbnail(file).is_ok();

        let elapsed_time = start_time.elapsed();
        println!("File {}: Took: {:.2?}", file, elapsed_time);

        result
    }).filter(|&result| result).count();

    println!("Successfully processed {}/{} files", success_count, total_files);
    Ok(())
}
