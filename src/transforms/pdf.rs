//! PDFTransform serializes/deserializes png data
use crate::transform::Transform;
use crate::error::{PipelineError, Result};
use lopdf::{Document, Object, Stream, dictionary};use std::io::Cursor;

#[derive(Debug)]
pub struct PdfTransform;

impl Transform for PdfTransform {
  fn encode(&self, data: Vec<u8>) -> Result<Vec<u8>> {
    let mut doc = Document::with_version("1.7");

    // Adding the pdf's visible text
    let visible_text = "Hello, World!";
    let text_stream_content = format!("BT /F1 24 Tf 100 700 Td ({}) Tj ET", visible_text);
    let text_stream = Stream::new(dictionary! {}, text_stream_content.as_bytes().to_vec());
    let text_stream_id = doc.add_object(text_stream);

    // PDF Page DIctionary
    let obj_id = doc.new_object_id();
    let page_id = doc.add_object(dictionary! {
      "Type" => "Page",
      "Parent" => obj_id,
      "Resources" => dictionary! {
        "Font" => dictionary! {
          "F1" => dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica"
          }
        }
      },
      "Contents" => text_stream_id,
      "MediaBox" => vec![0.into(), 0.into(), 600.into(), 800.into()],
    });

    // Creating /Pages dict
    let pages_id = doc.add_object(dictionary! {
      "Type" => "Pages",
      "Kids" => vec![page_id.into()],
      "Count" => 1,
    });

    // update page's parent
    if let Object::Dictionary(dict) = doc.get_object_mut(page_id).map_err(|e| PipelineError::Pdf(e.to_string()))? {
      dict.set("Parent", pages_id);
    }

    // Setup /Root catalog
    let catalog_id = doc.add_object(dictionary! {
      "Type" => "Catalog",
      "Pages" => pages_id,
    });
    doc.trailer.set(b"Root", catalog_id);

    // Adding the hidden stream
    let hidden_stream = Stream::new(
      dictionary! {
        "Type" => "XObject",
        "Subtype" => "Metadata",
        "Length" => data.len() as i64,
      },
      data,
    );
    let hidden_stream_id = doc.add_object(hidden_stream);

    // Store reference in /Info dir
    let info_id = doc.add_object(dictionary! {});
    doc.trailer.set("Info", info_id);
    let info_dict = doc.get_object_mut(info_id)
      .map_err(|e| PipelineError::Pdf(e.to_string()))?
      .as_dict_mut()
      .map_err(|e| PipelineError::Pdf(e.to_string()))?;
    info_dict.set("WackyPixels", hidden_stream_id);

    let mut pdf_bytes = Vec::new();
    
    doc.save_to(&mut Cursor::new(&mut pdf_bytes))
      .map_err(|e| e.to_string())?;
    Ok(pdf_bytes)
  }

  fn decode(&self, data: Vec<u8>) -> Result<Vec<u8>> {
    let doc = Document::load_from(Cursor::new(&data))
        .map_err(|e| PipelineError::Pdf(e.to_string()))?;

    // Find hidden stream using /Info dict
    let info_id = doc.trailer.get(b"Info")
      .map_err(|e| PipelineError::Pdf(e.to_string()))?
      .as_reference()
      .map_err(|e| PipelineError::Pdf(e.to_string()))?;
    let info_dict = doc.get_object(info_id)
      .map_err(|e| PipelineError::Pdf(e.to_string()))?
      .as_dict()
      .map_err(|e| PipelineError::Pdf(e.to_string()))?;
    let hidden_id = info_dict.get(b"WackyPixels")
      .map_err(|e| PipelineError::Pdf(e.to_string()))?
      .as_reference()
      .map_err(|e| PipelineError::Pdf(e.to_string()))?;

    // extract the hidden stream
    let hidden_object = doc.get_object(hidden_id)
      .map_err(|e| PipelineError::Pdf(e.to_string()))?;
    let hidden_stream = match hidden_object {
      Object::Stream(stream) => stream,
      _ => return Err(PipelineError::Pdf("WackyFiles is not a stream".to_string())),
    };
    
    let data = hidden_stream.content.clone();
  
    Ok(data)
  }


  fn name(&self) -> &str {
    "PDF"
  }

  fn extension(&self) -> &str {
    "pdf"
  }
}

