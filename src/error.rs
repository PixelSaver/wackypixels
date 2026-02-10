//! Storing custom errors to better track the errors happening where in the pipeline
use std::fmt;

#[derive(Debug)]
pub enum PipelineError {
  Io(std::io::Error),
  Image(String),
  Pdf(String),
  Lzma(String),
  Unicode(String),
  Wav(String),
  Utf8(std::string::FromUtf8Error),
  Flate(String),
  InvalidData(String),
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PipelineError::Io(e) => write!(f, "IO error: {}", e),
            PipelineError::Image(e) => write!(f, "Image encoding error: {}", e),
            PipelineError::Pdf(e) => write!(f, "PDF transformation error: {}", e),
            PipelineError::Lzma(e) => write!(f, "LZMA compression error: {}", e),
            PipelineError::Unicode(e) => write!(f, "Unicode encoding error: {}", e),
            PipelineError::Wav(e) => write!(f, "WAV audio error: {}", e),
            PipelineError::Utf8(e) => write!(f, "UTF-8 conversion error: {}", e),
            PipelineError::Flate(e) => write!(f, "Flate compression error: {}", e),
            PipelineError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}


impl std::error::Error for PipelineError {}

// Convenient conversions
impl From<std::io::Error> for PipelineError {
    fn from(e: std::io::Error) -> Self {
        PipelineError::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for PipelineError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        PipelineError::Utf8(e)
    }
}

// Helper for String errors
impl From<String> for PipelineError {
    fn from(s: String) -> Self {
        PipelineError::InvalidData(s)
    }
}

pub type Result<T> = std::result::Result<T, PipelineError>;