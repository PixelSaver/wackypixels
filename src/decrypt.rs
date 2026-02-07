use crate::image_data;

pub fn decrypt_image(path: &str, output_path: &str) -> Result<(), String> {
  let image = image_data::decode(path).map_err(|e| e.to_string())?;
  image.save(output_path).map_err(|e| e.to_string())?;
  Ok(())
}
