mod image_data;
mod encrypt;
mod decrypt;
mod pdf;
mod unicode;
mod lzma;
// mod lorem_ipsum;
mod wav;
use std::path::Path;

fn main() {
  encrypt::encrypt_image(Path::new("inputs/image.png"), Path::new("outputs/")).unwrap();
  decrypt::decrypt_image(Path::new("outputs/005.wav"), Path::new("outputs/")).unwrap();
}
