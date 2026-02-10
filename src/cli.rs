use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "wackypixels")]
#[command(about = "A wacky, cursed encoder meant to encode pngs.", long_about = None)]
#[command(version)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Encode an image through the pipeline
  Encode {
    /// Input file to encode
    #[arg(short, long, default_value = "inputs/image.png")]
    input: PathBuf,
    /// Output directory for encoding
    #[arg(short, long, default_value = "outputs")]
    output: PathBuf,
    /// Whether or not to seave intermediate files
    #[arg(short, long, default_value_t = true)]
    save_intermediates: bool,
    
    /// Custom pipeline (comma-separated)
    /// Example: image,pdf,lzma,unicode,wav
    #[arg(short, long, value_delimiter = ',')]
    pipeline: Option<Vec<TransformType>>,
  },
  /// Decode an image through the pipeline
  Decode {
    /// Path to encrypted file to decode
    #[arg(short, long, default_value = "inputs/image.png")]
    input: PathBuf,
    /// Output directory for decoding
    #[arg(short, long, default_value = "decrypted")]
    output: PathBuf,
    /// Whether or not to seave intermediate files
    #[arg(short, long, default_value_t = true)]
    save_intermediates: bool,
    
    /// Custom pipeline (comma-separated) in the forward direction
    /// Decoding happens in the reverse of whatever pipeline is given
    /// Example: image,pdf,lzma,unicode,wav
    #[arg(short, long, value_delimiter = ',')]
    pipeline: Option<Vec<TransformType>>,
    
    #[arg(short = 'f', long, default_value = "decrypted.png")]
    output_file: PathBuf,
  },
  
  /// Clean output directories
  Clean {
    /// Directories to clean 
    /// (Defaults to outputs/ and decrypted/)
    /// (Comma separated)
    #[arg(short, long, value_delimiter = ',')]
    dirs: Option<Vec<PathBuf>>,
    
    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    yes: bool,
  },
  
  /// List available transforms
  List,
  
  /// Run the default full pipeline (encode + decode)
  Run {
    /// Input file
    #[arg(short, long, default_value = "inputs/image.png")]
    input: PathBuf,
    
    /// Skip confirmation before cleaning
    #[arg(short = 'y', long)]
    yes: bool,
  },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TransformType {
  Image,
  Pdf,
  Lzma,
  Unicode,
  Wav,
}

impl TransformType {
  pub fn description(&self) -> &str {
    match self {
      TransformType::Image => "Image serialization (PNG -> binary)",
      TransformType::Pdf => "PDF, stored in the /Info metadata",
      TransformType::Lzma => "LZMA/XZ compression",
      TransformType::Unicode => "Unicode, multimode encoding (CJK, Emojis, Hidden characters, etc)",
      TransformType::Wav => "WAV audio encoding (amplitude modulation)",
    }
  }
}