//! ImageTransform serializes/deserializes png data
use crate::transform::Transform;
use image::{GenericImageView, ImageBuffer, ImageError, ImageFormat, ImageReader, Rgba};
use std::io::Cursor;

#[derive(Debug)]
pub struct ImageTransform;

impl Transform for ImageTransform {
  fn encode(&self, data: Vec<u8>) -> Result<Vec<u8>, String> {
    let img = ImageReader::new(Cursor::new(&data))
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

  fn decode(&self, data: Vec<u8>) -> Result<Vec<u8>, String> {
    if data.len() < 9 {
      return Err("Data too short".to_string());
    }

    let width = u32::from_le_bytes(data[0..4].try_into().unwrap());
    let height = u32::from_le_bytes(data[4..8].try_into().unwrap());
    let format = data[8];

    if format != 4 {
      return Err("Unsupported format, only RGBA supported".to_string());
    }

    let expected_len = 9 + (width * height * 4) as usize;
    if data.len() != expected_len {
      return Err("Pixel data length mismatch".to_string());
    }

    let pixels = data[9..].to_vec();

    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, pixels)
      .ok_or_else(|| "Failed to construct image buffer".to_string())?;

    let mut png_bytes: Vec<u8> = Vec::new();

    img
      .write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)
      .map_err(|e| e.to_string())?;

    Ok(png_bytes)
  }

  fn name(&self) -> &str {
    "PNG serialization"
  }

  fn extension(&self) -> &str {
    "png"
  }
}
