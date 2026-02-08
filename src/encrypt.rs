use std::path::{Path};
use std::fs;
use crate::{image_data, pdf};

pub fn encrypt_image(input: &Path, output: &Path) -> Result<(), String> {
  let output_dir = output.parent().ok_or("Output directory not found")?;
  
  // Serialize
  let data = image_data::encode(input).map_err(|e| e.to_string())?;
  fs::write(output, data.clone()).map_err(|e| e.to_string())?;
  
  // PDF
  let pdf_path = output_dir.join("001.pdf");
  pdf::encode_pdf(data, &pdf_path)
    .map_err(|e| e.to_string())?;
  Ok(())
}