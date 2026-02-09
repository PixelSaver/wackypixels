use crate::transform::Transform;
use crate::error::{PipelineError, Result};

#[derive(Clone, Copy)]
enum EncodeMode {
  /// Encodes two bits of information: 00
  Invisible,
  /// Encodes two bits of information: 01
  Cjk,
  /// Encodes two bits of information: 10
  Emoji,
  /// Encodes two bits of information: 11
  Alphanum,
}
impl EncodeMode {
  /// Returns (base_codepoint, payload_bits)
  fn config(&self) -> (u32, u32) {
    match self {
      EncodeMode::Invisible => (0xE0100, 7), 
      EncodeMode::Cjk => (0x4E00, 14),
      EncodeMode::Emoji => (0x1F600, 6),
      EncodeMode::Alphanum => (0x1D400, 10),
    }
  }
  /// Map 2-bit value to mode
  fn from_bits(bits: u8) -> Self {
      match bits & 0b11 {
          0b00 => EncodeMode::Invisible,
          0b01 => EncodeMode::Cjk,
          0b10 => EncodeMode::Emoji,
          0b11 => EncodeMode::Alphanum,
          _ => unreachable!(),
      }
  }
}

#[derive(Debug)]
pub struct UnicodeTransform;

impl Transform for UnicodeTransform {
    fn encode(&self, data: Vec<u8>) -> Result<Vec<u8>> {
      let mut out = String::new();
      let mut bit_buffer: u64 = 0;
      let mut bit_count: u32 = 0;
      let mut idx = 0;
      
      // Header: 3 × 14-bit Visible chars store total length
      let total_len = data.len() as u32;
      for i in 0..3 {
        let chunk = (total_len >> (i * 14)) & 0x3FFF;
        out.push(char::from_u32(0x4E00 + chunk).unwrap());
      }
      
      while idx < data.len() || bit_count > 0 {
        // Fill buffer
        while bit_count < 18 && idx < data.len() {
          bit_buffer = (bit_buffer << 8) | data[idx] as u64;
          bit_count += 8;
          idx += 1;
        }
        
        // Take 2 bits for mode (or pad if not enough bits)
        let mode_bits = if bit_count >= 2 {
          ((bit_buffer >> (bit_count - 2)) & 0b11) as u8
        } else {
          ((bit_buffer << (2 - bit_count)) & 0b11) as u8
        };
        bit_count = bit_count.saturating_sub(2);
        
        let mode = EncodeMode::from_bits(mode_bits);
        let (base, depth) = mode.config();
        
        // Take depth bits for payload
        let val = if bit_count >= depth {
          let v = (bit_buffer >> (bit_count - depth)) & ((1 << depth) - 1);
          bit_count -= depth;
          v
        } else {
          // pad remaining bits
          let v = (bit_buffer << (depth - bit_count)) & ((1 << depth) - 1);
          bit_count = 0;
          v
        };
        
        bit_buffer &= (1 << bit_count) - 1;
        out.push(char::from_u32(base + val as u32).unwrap());
      }
      
      Ok(out.as_bytes().to_vec())
    }
    
    fn decode(&self, data: Vec<u8>) -> Result<Vec<u8>> {
      let encoded = String::from_utf8(data)?;
      let mut chars = encoded.chars();
      
      // Decode length header
      let mut total_len: u32 = 0;
      for i in 0..3 {
        let c = chars.next().ok_or("Missing header")
          .map_err(|e| PipelineError::Unicode(e.to_string()))?;
        let v = (c as u32)
          .checked_sub(0x4E00)
          .ok_or("Invalid header char")
          .map_err(|e| PipelineError::Unicode(e.to_string()))?;
        total_len |= v << (i * 14);
      }
      
      let mut out = Vec::with_capacity(total_len as usize);
      let mut bit_buffer: u64 = 0;
      let mut bit_count: u32 = 0;
      
      for c in chars {
        let cp = c as u32;
        // Determine mode and payload bits
        let (base, depth) = if (0x4E00..=0x9FFF).contains(&cp) {
          (0x4E00, 14)
        } else if (0x1F600..=0x1F63F).contains(&cp) {
          (0x1F600, 6)
        } else if (0xE0100..=0xE01EF).contains(&cp) {
          (0xE0100, 7)
        } else if (0x1D400..=0x1D7FF).contains(&cp) {
          (0x1D400, 10)
        } else {
          continue;
        };
        
        let val = cp - base;
        
        // Prepend mode bits (the 2 bits) for this glyph
        let mode_bits = match base {
          0x4E00 => 0b01,
          0xE0100 => 0b00,
          0x1F600 => 0b10,
          0x1D400 => 0b11,
          _ => unreachable!(),
        };
        bit_buffer = (bit_buffer << (depth + 2)) | ((mode_bits as u64) << depth) | val as u64;
        bit_count += depth + 2;
        
        // Extract bytes
        while bit_count >= 8 && out.len() < total_len as usize {
          let byte = (bit_buffer >> (bit_count - 8)) as u8;
          out.push(byte);
          bit_count -= 8;
          bit_buffer &= (1 << bit_count) - 1;
        }
      }
      
      if out.len() != total_len as usize {
        return Err(PipelineError::Unicode("Decoded length mismatch".into()));
      }
      Ok(out)
    }
    
    fn name(&self) -> &str {
        "Unicode Encoding"
    }
    
    fn extension(&self) -> &str {
        "txt"
    }
}