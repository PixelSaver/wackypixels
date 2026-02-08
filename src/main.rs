mod image_data;
mod encrypt;
mod decrypt;
mod pdf;
use std::path::Path;

fn main() {
  encrypt::encrypt_image(Path::new("inputs/image.png"), Path::new("outputs/encrypted.png")).unwrap();
  decrypt::decrypt_image(Path::new("outputs/001.pdf"), Path::new("outputs/decrypted.png")).unwrap();
}
