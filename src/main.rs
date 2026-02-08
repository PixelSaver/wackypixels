mod image_data;
mod encrypt;
mod decrypt;
mod pdf;
mod unicode;
mod lorem_ipsum;
use std::path::Path;

fn main() {
  encrypt::encrypt_image(Path::new("inputs/image.png"), Path::new("outputs/")).unwrap();
  decrypt::decrypt_image(Path::new("outputs/003.txt"), Path::new("outputs/")).unwrap();
}
