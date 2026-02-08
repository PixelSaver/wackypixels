use xz2::write::{XzEncoder, XzDecoder};
use std::io::{Write};

pub fn compress_lzma(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut encoder = XzEncoder::new(Vec::new(), 9);
    encoder.write_all(data).map_err(|e| e.to_string())?;
    encoder.finish().map_err(|e| e.to_string())
}

pub fn decompress_lzma(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = XzDecoder::new(Vec::new());
    decoder.write_all(data).map_err(|e| e.to_string())?;
    decoder.finish().map_err(|e| e.to_string())
}