use crate::error::{PipelineError, Result};
use crate::transform::Transform;
use std::path::{Path, PathBuf};
use std::fs;

pub struct Pipeline {
  transforms: Vec<Box<dyn Transform>>,
  save_intermediates: bool,
}

impl Pipeline {
  pub fn new() -> Self {
    Self {
      transforms: vec![],
      save_intermediates: false,
    }
  }
  
  pub fn add<T: Transform + 'static>(mut self, transform: T) -> Self {
    self.transforms.push(Box::new(transform));
    self
  }
  
  pub fn save_intermediates(mut self, enable: bool) -> Self {
    self.save_intermediates = enable;
    self
  }
  
  pub fn encode(&self, input: &Path, output_dir: &Path) -> Result<PathBuf> {
    fs::create_dir_all(output_dir)?;
    
    let mut data = fs::read(input)
      .map_err(|e| {
        eprintln!("Failed to read input file: {}", input.display());
        e
      })?;
    
    let total = self.transforms.len();
    
    for (i, transform) in self.transforms.iter().enumerate() {
      let step = i + 1;
      
      println!("[{}/{}] Applying: {}", step, total, transform.name());
      
      // Give errors context
      data = transform.encode(data).map_err(|e| {
        eprintln!("X Failed at step {}/{}: {}", step, total, transform.name());
        eprintln!("  Error: {}", e);
        e
      })?;
      
      if self.save_intermediates {
        let filename = format!("{:03}_{}.{}", 
          step, 
          transform.name().replace(" ", "_").to_lowercase(),
          transform.extension(),
        );
        let path = output_dir.join(filename);
        fs::write(&path, &data)
          .map_err(|e| {
            eprintln!("Failed to save intermediate file: {}", path.display());
            e
          })?;
        println!("Saved: {}", path.display());
      }
      
      println!(" Output size: {} bytes", data.len());
    }
    
    let final_ext = self.transforms.last()
      .map(|t| t.extension())
      .unwrap_or("bin");
    let output_path = output_dir.join(format!("encrypted.{}", final_ext));
    fs::write(&output_path, data)?;
    
    println!("\n Encryption complete: {}", output_path.display());
    Ok(output_path)
  }
  
  pub fn decode(&self, input: &Path, output_dir: &Path) -> Result<PathBuf> {
    fs::create_dir_all(output_dir)?;
    
    let mut data = fs::read(input)
      .map_err(|e| {
        eprintln!("Failed to read encrypted file: {}", input.display());
        e
      })?;
    let total = self.transforms.len();
    
    for (i, transform) in self.transforms.iter().rev().enumerate() {
      let step = i + 1;
      
      println!("[{}/{}] Reversing: {}", step, total, transform.name());
      
      data = transform.decode(data).map_err(|e| {
        eprintln!("X Failed at decode step {}/{}: {}", step, total, transform.name());
        eprintln!("  Error: {}", e);
        eprintln!("  This could be a number of things:");
        eprintln!("    - Corrupted data at this stage");
        eprintln!("    - Wrong pipeline order");
        eprintln!("    - Missing transformation step");
        e
      })?;
      
      if self.save_intermediates {
        let extension = if i + 1 < total {
          self.transforms[total - i - 2].extension() 
        } else {
          "png"
        };
        let filename = format!("{:03}_{}_decoded.{}",
          total-i,
          transform.name().replace(" ", "_").to_lowercase(),
          extension
        );
        let path = output_dir.join(filename);
        fs::write(&path, &data)
          .map_err(|e| {
            eprintln!("Failed to save intermediate file: {}", path.display());
            e
          })?;
        println!("  Saved: {}", path.display());
      }
      
      println!("  Output size: {} bytes", data.len());
    }
    
    let output_path = output_dir.join("decrypted.png");
    fs::write(&output_path, data)?;
    
    println!("\n Decryption complete: {}", output_path.display());
    Ok(output_path)
    
  }
}