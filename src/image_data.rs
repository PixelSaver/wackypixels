//! ImageData is a struct that reads data from an image file
//! 
//! # Examples
//! 
//! ```
//! use wackypixels::image_reader::ImageReader;
//! 
//! let reader = ImageReader::new("path/to/image.png");
//! let data = reader.read();
//! ```
use image::{DynamicImage, GenericImageView, ImageEncoder, ImageReader};

struct ImageData {
}

impl ImageData {
  /// Returns serialized data from image file
  pub fn read(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = ImageReader::open(path)
      .expect("Failed to open image file")
      .decode()
      .expect("Failed to decode image");
    let (width, hieght) = img.dimensions();
    let pixels = img.as_bytes();
    
    let mut out = Vec::new();
    
    out.extend_from_slice(&width.to_le_bytes());
    out.extend
  }
}
