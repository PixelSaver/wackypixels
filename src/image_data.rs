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
use image::{GenericImageView, ImageBuffer, ImageReader, Rgba};
use std::path::Path;
use std::fs;

/// Returns serialized data from image file
pub fn encode(path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
  let img = ImageReader::open(path)
    .expect("Failed to open image file")
    .decode()
    .expect("Failed to decode image");
  let (width, height) = img.dimensions();
  let pixels = img.to_rgba8().into_raw();
  
  let mut out = Vec::new();
  
  out.extend_from_slice(&width.to_le_bytes());
  out.extend_from_slice(&height.to_le_bytes());
  out.push(4); // RGBA
  
  out.extend_from_slice(&pixels);
  
  Ok(out)
}

pub fn decode(data: Vec<u8>) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
  if data.len() < 9 {
    return Err("Data too short".to_string())
  }
  
  let width = u32::from_le_bytes(data[0..4].try_into().unwrap());
  let height = u32::from_le_bytes(data[4..8].try_into().unwrap());
  let format = data[8];
  
  if format != 4 {
    return Err("Unsupported format, only RGBA supported".to_string())
  }
  
  let expected_len = 9 + (width * height * 4) as usize;
  if data.len() != expected_len {
    return Err("Pixel data length mismatch".to_string());
  }
  
  let pixels = data[9..].to_vec();
  
  ImageBuffer::from_raw(width, height, pixels)
    .ok_or("Failed to construct image buffer".to_string())
}
