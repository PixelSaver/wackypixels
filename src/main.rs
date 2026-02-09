mod image_data;
mod encrypt;
mod decrypt;
mod pdf;
mod unicode;
mod lzma;
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
mod wav;

// use pipeline::Pipeline;
use std::path::Path;
use transforms::*;
use error::Result;

fn main() {
  encrypt::encrypt_image(Path::new("inputs/image.png"), Path::new("outputs/")).unwrap();
  decrypt::decrypt_image(Path::new("outputs/005.wav"), Path::new("outputs/")).unwrap();
}
