use crate::{image_data, pdf};
use lopdf::Document;
use std::path::Path;

pub fn decrypt_image(input: &Path, output: &Path) -> Result<(), String> {
  // PDF
  let data = pdf::decode_pdf(input)
    .map_err(|e| e.to_string())?;
  
  let image = image_data::decode(data).map_err(|e| e.to_string())?;
  image.save(output).map_err(|e| e.to_string())?;
  Ok(())
}
