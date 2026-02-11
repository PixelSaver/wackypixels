mod transforms {
  pub mod image;
  pub mod pdf;
  pub mod lzma;
  pub mod unicode;
  pub mod wav;
  pub mod flate;
}
mod pipeline;
mod transform;
mod error;
mod cli;
mod pipeline_builder;

use clap::Parser;
use cli::*;
use std::{fs, io::{self, Write}};
use error::Result;
use std::path::PathBuf;

fn main() {
  if let Err(e) = run() {
    eprintln!("  Error: {}", e);
    std::process::exit(1);
  }
}
fn run() -> Result<()> {
  let cli = Cli::parse();
  
  match cli.command {
    Commands::Encode { input, output, save_intermediates, pipeline } => {
      let mut pipe = if let Some(types) = pipeline {
        pipeline_builder::build_custom_pipeline(&types)
      } else {
        pipeline_builder::build_default_pipeline()
      };
      
      pipe = pipe.save_intermediates(save_intermediates);
      println!("--- ENCODING ---");
      pipe.print_summary();
      
      pipe.encode(&input, &output)?;
    }
    
    Commands::Decode { input, output, save_intermediates, pipeline, output_file } => {
      let mut pipe = if let Some(types) = pipeline {
        pipeline_builder::build_custom_pipeline(&types)
      } else {
        pipeline_builder::build_default_pipeline()
      };
      
      pipe = pipe.save_intermediates(save_intermediates);
      
      println!("--- DECODING ---");
      pipe.print_summary();
      
      pipe.decode(&input, &output, Some(&output_file))?;
    }
    
    Commands::Clean { dirs, yes } => {
      let directories = dirs.unwrap_or_else(|| {
        vec!["outputs".into(), "decrypted".into()]
      });
      
      if !yes {
        println!("About to delete:");
        for dir in &directories {
          if dir.exists() {
            println!("  - {}/", dir.display());
          }
        }
        println!("\nContinue? [y/N] ");
        io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        
        if !response.trim().eq_ignore_ascii_case("y") {
          println!("Cancelled.");
          return Ok(());
        }
      }
      
      for dir in directories {
        if dir.exists() {
          fs::remove_dir_all(&dir)
            .map_err(|e| format!("Failed to remove{}: {}", dir.display(), e))?;
          println!("  Removed {}/", dir.display());
        } else {
          println!("X {} (doesn't exist)", dir.display());
        }
      }
      
      println!("Cleaned!")
    }
    
    Commands::List => {
      println!("Available transforms: \n");
      
      let all_types = [
          TransformType::Image,
          TransformType::Pdf,
          TransformType::Lzma,
          TransformType::Unicode,
          TransformType::Wav,
          TransformType::Gzip,
      ];
      
      for t in all_types {
        println!("  {:14} - {}", format!("{:?}", t).to_lowercase(), t.description());
      }
      
      println!("\nExample usage:");
      println!("  wackypixels encode --pipeline image,lzma,unicode")
    }
    
    Commands::Run { input, encode_output, decode_output, output_file, pipeline, save_intermediates, yes } => {
      println!("!! Running Full Pipeline\n");
      
      // Clean first
      let dirs: Vec<PathBuf> = vec!["outputs".into(), "decrypted".into()];
      if !yes {
        print!("Clean output directories first? [Y/n] ");
        io::stdout().flush().unwrap();
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        
        if !response.trim().eq_ignore_ascii_case("n") {
          for dir in &dirs {
            if dir.exists() {
              fs::remove_dir_all(&dir)
                .map_err(|e| format!("Failed to remove{}: {}", dir.display(), e))?;
              println!("  Removed {}/", dir.display());
            }
          }
          println!("  Cleaned!\n");
        }
      } else {
        for dir in &dirs {
          if dir.exists() {
            fs::remove_dir_all(dir)
              .map_err(|e| format!("Failed to remove{}: {}", dir.display(), e))?;
            println!("  Removed {}/", dir.display());
          }
        }
      }
      
      // Encode
      let mut pipeline = if let Some(types) = pipeline {
        pipeline_builder::build_custom_pipeline(&types)
      } else {
        pipeline_builder::build_default_pipeline()
      };
      pipeline = pipeline.save_intermediates(save_intermediates);
      
      println!("--- ENCODING ---");
      pipeline.print_summary();
      
      let encrypted = pipeline.encode(&input, &encode_output)?;
      
      println!("\n{}\n", "-".repeat(60));
      
      // Decode
      println!("--- DECODING ---");
      pipeline.print_summary();
      
      let decrypted = pipeline.decode(&encrypted, &decode_output, Some(&output_file))?;
      
      println!("--- SUCCESS ---");
      println!("Original:  {}", input.display());
      println!("Encrypted: {}", encrypted.display());
      println!("Decrypted: {}", decrypted.display());
    }
  }
  
  Ok(())
}
