mod image_data;
mod encrypt;
mod decrypt;


fn main() {
  encrypt::encrypt_image("inputs/image.png", "outputs/encrypted.png").unwrap();
  decrypt::decrypt_image("outputs/encrypted.png", "outputs/decrypted.png").unwrap();
}
