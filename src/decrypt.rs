use crate::{image_data, lzma, pdf, unicode, wav};
use std::{fs, path::Path};

pub fn decrypt_image(input: &Path, output_dir: &Path) -> Result<(), String> {
  // Wav
  let w_dir = &output_dir.join("006.txt");
  let wav_data = fs::read(input).map_err(|e| e.to_string())?;
  let wav_txt = wav::decode_from_audio(&wav_data)
    .map_err(|e| e.to_string())?;
  fs::write(w_dir, wav_txt).map_err(|e| e.to_string())?;
  
  // Unicode
  let txt = fs::read_to_string(w_dir).map_err(|e| e.to_string())?;
  let u_data = unicode::decode_from_unicode(&txt)
    .map_err(|e| e.to_string())?;
  let u_dir = &output_dir.join("007.xz");
  fs::write(u_dir, u_data.clone()).map_err(|e| e.to_string())?;
  
  // Zip
  let z_dir = &output_dir.join("008.pdf");
  let z_data = lzma::decompress_lzma(&u_data)?;
  fs::write(z_dir, z_data.clone()).map_err(|e| e.to_string())?;
  
  // PDF
  let data = pdf::decode_pdf(z_dir)
    .map_err(|e| e.to_string())?;
  
  let i_dir = &output_dir.join("decrypted.png");
  let image = image_data::decode(data).map_err(|e| e.to_string())?;
  image.save(i_dir).map_err(|e| e.to_string())?;
  Ok(())
}
