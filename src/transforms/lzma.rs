use xz2::write::{XzEncoder, XzDecoder};
use std::io::{Write};
use crate::transform::Transform;
use crate::error::{PipelineError, Result};

#[derive(Debug)]
pub struct LzmaTransform;

impl Transform for LzmaTransform {
    fn encode(&self, data: Vec<u8>) -> Result<Vec<u8>> {
      let mut encoder = XzEncoder::new(Vec::new(), 9);
      encoder.write_all(&data).map_err(|e| e.to_string())?;
      encoder.finish().map_err(|e| PipelineError::Lzma(e.to_string()))
    }
    
    fn decode(&self, data: Vec<u8>) -> Result<Vec<u8>> {
      let mut decoder = XzDecoder::new(Vec::new());
      decoder.write_all(&data).map_err(|e| e.to_string())?;
      decoder.finish().map_err(|e| PipelineError::Lzma(e.to_string()))
    }
    
    fn name(&self) -> &str {
        "LZMA Compression"
    }
    
    fn extension(&self) -> &str {
        "xz"
    }
}