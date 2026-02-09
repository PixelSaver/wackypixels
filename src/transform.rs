use std::fmt::Debug;

/// A reversible transformation step in the wacky encoding pipeline
pub trait Transform: Debug {
  /// Apply the transformation (encode direction)
  fn encode(&self, data: Vec<u8>) -> Result<Vec<u8>, String>;
  
  /// Reverse the transformation (decode direction)
  fn decode(&self, data: Vec<u8>) -> Result<Vec<u8>, String>;
  
  fn name(&self) -> &str;
  
  /// File extension for intermediate outputs
  fn extension(&self) -> &str {
    "bin"
  }
}