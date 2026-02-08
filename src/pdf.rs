use lopdf::{Document, Object, Stream, dictionary};
use std::path::Path;
use std::fs;
use std::io::Read;

pub fn encode_pdf(data: Vec<u8>, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
  let mut doc = Document::with_version("1.7");
  
  // Adding the pdf's visible text
  let visible_text = "Hello, World!";
  let text_stream_content = format!(
    "BT /F1 24 Tf 100 700 Td ({}) Tj ET",
    visible_text,
  );
  let text_stream = Stream::new(dictionary!{}, text_stream_content.as_bytes().to_vec());
  let text_stream_id = doc.add_object(text_stream);
  
  // PDF Page DIctionary
  let obj_id = doc.new_object_id();
  let page_id = doc.add_object(
    dictionary! {
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
    }
  );
  
  // Creating /Pages dict
  let pages_id = doc.add_object(dictionary! {
    "Type" => "Pages",
    "Kids" => vec![page_id.into()],
    "Count" => 1,
  });
  
  // update page's parent
  if let Object::Dictionary(dict) = doc.get_object_mut(page_id)? {
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
  let info_dict = doc.get_object_mut(info_id)?.as_dict_mut()?;
  info_dict.set("WackyPixels", hidden_stream_id);
  
  doc.save(output_path)?;
  Ok(())
}

pub fn decode_pdf(input_path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
  // DEBUG: Print the first 5 bytes
  let mut f = std::fs::File::open(input_path)?;
  let mut header = [0u8; 5];
  f.read_exact(&mut header)?;
  println!("File Header: {:?}", String::from_utf8_lossy(&header));
  
  let doc = Document::load(input_path)?;
  
  // Find hidden stream using /Info dict
  let info_id = doc.trailer.get(b"Info")?.as_reference()?;
  let info_dict = doc.get_object(info_id)?.as_dict()?;
  let hidden_id = info_dict.get(b"WackyPixels")?.as_reference()?;
  
  // extract the hidden stream
  let hidden_object = doc.get_object(hidden_id)?;
  let hidden_stream = match hidden_object {
    Object::Stream(stream) => stream,
  _ => return Err("WackyFiles is not a stream".into()),
  };
  
  let data = hidden_stream.content.clone();
  
  Ok(data)
}
