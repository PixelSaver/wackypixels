mod transforms {
  pub mod image;
  pub mod pdf;
  pub mod lzma;
  pub mod unicode;
  pub mod wav;
}
mod pipeline;
mod transform;
mod error;

use pipeline::Pipeline;
use std::path::Path;
use transforms::*;
use error::Result;

fn main() {
  if let Err(e) = run() {
    eprintln!("\nX Pipeline failed!");
    eprintln!("  Error: {}", e);
    std::process::exit(1);
  }
}
fn run() -> Result<()> {
  let pipeline = Pipeline::new()
    .add(image::ImageTransform)
    .add(pdf::PdfTransform)
    .add(lzma::LzmaTransform)
    .add(unicode::UnicodeTransform)
    .add(wav::WavTransform)
    .save_intermediates(true);
  
  println!("--- ENCODING ---");
  let output = pipeline.encode(
    Path::new("inputs/image.png"),
    Path::new("outputs")
  )?;
  
  println!("--- DECODING ---");
  let decrypted = pipeline.decode(
    &output,
    Path::new("decrypted")
  )?;
  
  println!("--- SUCCESS ---");
  println!("Original: inputs/image.png");
  println!("Encrypted: {}", output.display());
  println!("Decrypted: {}", decrypted.display());
  
  Ok(())
}
