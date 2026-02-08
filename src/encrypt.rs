use std::path::{Path};
use std::fs;
use crate::{image_data, lzma, pdf, unicode, wav};

pub fn encrypt_image(input: &Path, output: &Path) -> Result<(), String> {
  // Serialize
  let data = image_data::encode(input).map_err(|e| e.to_string())?;
  let data_path = output.join("001.bin");
  fs::write(data_path, data.clone()).map_err(|e| e.to_string())?;
  
  // PDF
  let pdf_path = output.join("002.pdf");
  pdf::encode_pdf(data, &pdf_path)
    .map_err(|e| e.to_string())?;
  
  // Zip
  let zip_path = output.join("003.xz");
  let pdf_data = fs::read(&pdf_path).map_err(|e| e.to_string())?;
  let zip_data = lzma::compress_lzma(&pdf_data)?;
  fs::write(zip_path, zip_data.clone()).map_err(|e| e.to_string())?;
  
  // Unicode
  let unicode_path = output.join("004.txt");
  let unicode_data = unicode::encode_to_unicode(zip_data);
  fs::write(unicode_path, unicode_data.clone()).map_err(|e| e.to_string())?;
  
  // wav
  let wav_data = wav::encode_to_audio(unicode_data.as_bytes());
  fs::write(output.join("005.wav"), wav_data).map_err(|e| e.to_string())?;
  
  Ok(())
}