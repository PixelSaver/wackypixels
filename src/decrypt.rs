use crate::{image_data, pdf, unicode};
use std::{fs, path::Path};

pub fn decrypt_image(input: &Path, output_dir: &Path) -> Result<(), String> {
  // Unicode
  let txt = fs::read_to_string(input).map_err(|e| e.to_string())?;
  let u_data = unicode::decode_from_unicode(&txt)
    .map_err(|e| e.to_string())?;
  let u_dir = &output_dir.join("004.pdf");
  fs::write(u_dir, u_data).map_err(|e| e.to_string())?;
  
  // PDF
  let data = pdf::decode_pdf(u_dir)
    .map_err(|e| e.to_string())?;
  
  let i_dir = &output_dir.join("decrypted.png");
  let image = image_data::decode(data).map_err(|e| e.to_string())?;
  image.save(i_dir).map_err(|e| e.to_string())?;
  Ok(())
}
