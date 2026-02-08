use std::path::{Path};
use std::fs;
use crate::{image_data, pdf, unicode, lorem_ipsum::LOREM_IPSUM};

pub fn encrypt_image(input: &Path, output: &Path) -> Result<(), String> {
  // Serialize
  let data = image_data::encode(input).map_err(|e| e.to_string())?;
  let data_path = output.join("001.bin");
  fs::write(data_path, data.clone()).map_err(|e| e.to_string())?;
  
  // PDF
  let pdf_path = output.join("002.pdf");
  pdf::encode_pdf(data, &pdf_path)
    .map_err(|e| e.to_string())?;
  
  // Unicode
  let unicode_path = output.join("003.txt");
  let pdf_data = fs::read(&pdf_path).map_err(|e| e.to_string())?;
  let unicode_data = unicode::encode_to_unicode(pdf_data);
  fs::write(unicode_path, unicode_data).map_err(|e| e.to_string())?;
  
  Ok(())
}