use std::fs;
use crate::image_data;

pub fn encrypt_image(path: &str, output_path: &str) -> Result<(), String> {
  let data = image_data::encode(path).map_err(|e| e.to_string())?;
  fs::write(output_path, data).map_err(|e| e.to_string())?;
  Ok(())
}