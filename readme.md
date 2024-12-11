# DICOM Thumbnail Generator

A Rust application that recursively processes DICOM files and generates thumbnail images in JPG format.

## Features

- Recursively scans directories for DICOM files (`.dcm` extension)
- Generates 150x150 pixel thumbnails using Lanczos3 filtering
- Handles errors gracefully, continuing processing even if individual files fail
- Provides progress feedback and timing information
- Outputs thumbnails in a dedicated `thumbnails` directory

## Prerequisites

- Rust (latest stable version)
- GDCM library (required for DICOM pixel data decoding)

### Installing GDCM

#### Ubuntu/Debian

```bash
sudo apt-get install libgdcm-dev
```

#### macOS

```bash
brew install gdcm
```

## Usage
Just change the `DICOM_DIR` variable in the `src/main.rs` file to the path of the folder you want to process.

## Run

```bash
cargo run --release
```

The program will:
- Scan the specified directory recursively for `.dcm` files
- Create a `thumbnails` directory in the current working directory
- Generate JPG thumbnails for each DICOM file
- Display progress and timing information for each processed file

## Output

- Thumbnails are saved in the `thumbnails` directory
- Each thumbnail is named after its source DICOM file with a `.jpg` extension
- Final summary shows the number of successfully processed files

## Error Handling

The application handles various error conditions:
- Invalid DICOM files
- Corrupted pixel data
- File system permission issues
- Invalid image formats

Errors are logged to the console, but processing continues for remaining files.

## Dependencies

- `dicom` (0.8): DICOM file parsing
- `dicom-pixeldata` (0.8): Pixel data decoding with GDCM support
- Standard Rust libraries for file system operations and timing
- CMAKE

## License
MIT
