use flate2::write::{GzEncoder, GzDecoder};
use flate2::Compression;
use image::EncodableLayout;
use std::io::Write;
use crate::transform::Transform;
use crate::error::{PipelineError, Result};

#[derive(Debug)]
pub struct GzipTransform;

impl Transform for GzipTransform {
  fn encode(&self, data: Vec<u8>) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
      .write_all(&data)
      .map_err(|e| PipelineError::Flate(e.to_string()))?;
    Ok(
      encoder
        .finish()
        .map_err(|e| PipelineError::Flate(e.to_string()))?
        .as_bytes()
        .to_vec()
    )
  }
  
  fn decode(&self, data: Vec<u8>) -> Result<Vec<u8>> { 
    let mut decoder = GzDecoder::new(Vec::new());
    decoder
        .write_all(&data)
        .map_err(|e| PipelineError::Flate(e.to_string()))?;
    decoder
        .finish()
        .map_err(|e| PipelineError::Flate(e.to_string()))
  }
  
  fn name(&self) -> &str {
    "Gzip Compression"
  }
  
  fn extension(&self) -> &str {
    "gz"
  }
}
