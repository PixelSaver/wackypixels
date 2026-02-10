use crate::cli::TransformType;
use crate::pipeline::Pipeline;
use crate::transforms::*;

pub fn build_default_pipeline() -> Pipeline {
  Pipeline::new()
    .add(image::ImageTransform)
    .add(pdf::PdfTransform)
    .add(lzma::LzmaTransform)
    .add(unicode::UnicodeTransform)
    .add(wav::WavTransform)
}

pub fn build_custom_pipeline(types: &[TransformType]) -> Pipeline {
  let mut pipeline = Pipeline::new();
  
  for t in types {
    match t {
      TransformType::Image => pipeline = pipeline.add(image::ImageTransform),
      TransformType::Pdf => pipeline = pipeline.add(pdf::PdfTransform),
      TransformType::Lzma => pipeline = pipeline.add(lzma::LzmaTransform),
      TransformType::Unicode => pipeline = pipeline.add(unicode::UnicodeTransform),
      TransformType::Wav => pipeline = pipeline.add(wav::WavTransform),
      TransformType::Gzip => pipeline = pipeline.add(flate::GzipTransform),
    }
  }
  
  pipeline
}