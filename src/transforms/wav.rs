use hound::{WavWriter, WavSpec};
use crate::transform::Transform;
use crate::error::{PipelineError, Result};
use std::io::Cursor;

fn qam16_map(symbol: u8) -> (i8, i8) {
  // 16-QAM constellation points
  match symbol {
    0x0 => (-96, -96),   0x1 => (-96, -32),
    0x2 => (-96,  32),   0x3 => (-96,  96),
    0x4 => (-32, -96),   0x5 => (-32, -32),
    0x6 => (-32,  32),   0x7 => (-32,  96),
    0x8 => ( 32, -96),   0x9 => ( 32, -32),
    0xA => ( 32,  32),   0xB => ( 32,  96),
    0xC => ( 96, -96),   0xD => ( 96, -32),
    0xE => ( 96,  32),   0xF => ( 96,  96),
    _ => unreachable!(),
  }
}

fn qam16_demap(i: i8, q: i8) -> u8 {
  // Find closest constellation point
  // Thresholds at -64, 0, 64 for 4 levels: -96, -32, 32, 96

  let i_level = match i {
    -128..=-64 => 0, // -96
    -63..=0    => 1, // -32
    1..=64     => 2, //  32
    65..=127   => 3, //  96
  };

  let q_level = match q {
    -128..=-64 => 0, // -96
    -63..=0    => 1, // -32
    1..=64     => 2, //  32
    65..=127   => 3, //  96
  };

  // Map back to 4-bit symbol
  (i_level << 2) | q_level
}

#[derive(Debug)]
pub struct WavTransform;

impl Transform for WavTransform {
  fn encode(&self, data: Vec<u8>) -> Result<Vec<u8>> {
    let samples_per_symbol = 2; // Nyquist limit

    let spec = WavSpec {
      channels: 2, // I/Q channels for QAM
      sample_rate: 8000,
      bits_per_sample: 8,
      sample_format: hound::SampleFormat::Int,
    };

    let mut cursor = Cursor::new(Vec::new());
    let mut writer = WavWriter::new(&mut cursor, spec).unwrap();

    // Header
    let len = data.len() as u32;
    for i in 0..32 {
      let bit = ((len >> i) & 1) as i8;
      let val = if bit == 1 { 127 } else { -127 };

      // Repeat samples_per_symbol times (matching the data encoding)
      for _ in 0..samples_per_symbol {
        writer.write_sample(val).unwrap();  // I channel
        writer.write_sample(val).unwrap();  // Q channel
      }
    }

    // Each symbol encodes 4 bits (16-QAM constellation)
    for chunk in data.chunks(1) {
      let byte = chunk[0];

      // High nibble
      let high = (byte >> 4) & 0x0F;
      let (i_high, q_high) = qam16_map(high);
      for _ in 0..samples_per_symbol {
        writer.write_sample(i_high).unwrap();
        writer.write_sample(q_high).unwrap();
      }

      // Low nibble
      let low = byte & 0x0F;
      let (i_low, q_low) = qam16_map(low);
      for _ in 0..samples_per_symbol {
        writer.write_sample(i_low).unwrap();
        writer.write_sample(q_low).unwrap();
      }
    }

    writer.finalize().unwrap();
    Ok(cursor.into_inner())
  }

  fn decode(&self, data: Vec<u8>) -> Result<Vec<u8>> {
    let mut reader = hound::WavReader::new(Cursor::new(data))
      .map_err(|e| e.to_string())?;

    let samples: Vec<i8> = reader.samples::<i8>()
      .map(|s| s.map_err(|e| PipelineError::Wav(e.to_string())))
      .collect::<Result<Vec<_>>>()
      .map_err(|e| PipelineError::Wav(e.to_string()))?;

    let samples_per_symbol = 2;

    println!("Total samples: {}", samples.len());

    // Decode length from header
    let mut len: u32 = 0;
    for i in 0..32 {
      let base_idx = i * samples_per_symbol * 2;

      // Average all I samples in this bit period
      let mut sum = 0i32;
      for s in 0..samples_per_symbol {
        sum += samples[base_idx + s * 2] as i32;
      }
      let avg = sum / samples_per_symbol as i32;

      let bit = if avg > 0 { 1 } else { 0 };
      len |= bit << i;
    }

    // Calculate expected samples needed
    let header_samples = 32 * samples_per_symbol * 2;
    let data_samples = len as usize * 2 * samples_per_symbol * 2; // 2 symbols per byte
    let expected_total = header_samples + data_samples;
    
    if samples.len() < expected_total {
      return Err(PipelineError::Wav(format!(
        "Not enough samples: have {}, need {}",
        samples.len(), expected_total
      )));
    }

    let mut output = Vec::with_capacity(len as usize);
    let mut idx = header_samples;

    for byte_num in 0..len {
      let mut byte = 0u8;

      // Decode high nibble - average samples
      if idx + samples_per_symbol * 2 > samples.len() {
        return Err(PipelineError::Wav(format!(
          "Out of bounds at byte {} high nibble: idx={}, len={}",
          byte_num, idx, samples.len()
        )));
      }

      let mut i_sum = 0i32;
      let mut q_sum = 0i32;
      for s in 0..samples_per_symbol {
        i_sum += samples[idx + s * 2] as i32;
        q_sum += samples[idx + s * 2 + 1] as i32;
      }
      let i_high = (i_sum / samples_per_symbol as i32) as i8;
      let q_high = (q_sum / samples_per_symbol as i32) as i8;

      let high_nibble = qam16_demap(i_high, q_high);
      byte |= high_nibble << 4;

      idx += samples_per_symbol * 2;

      // Decode low nibble - average samples
      if idx + samples_per_symbol * 2 > samples.len() {
        return Err(
          PipelineError::Wav(format!(
            "Out of bounds at byte {} low nibble: idx={}, len={}, bytes decoded so far: {}",
            byte_num, idx, samples.len(), output.len()
          )));
      }

      let mut i_sum = 0i32;
      let mut q_sum = 0i32;
      for s in 0..samples_per_symbol {
        i_sum += samples[idx + s * 2] as i32;
        q_sum += samples[idx + s * 2 + 1] as i32;
      }
      let i_low = (i_sum / samples_per_symbol as i32) as i8;
      let q_low = (q_sum / samples_per_symbol as i32) as i8;

      let low_nibble = qam16_demap(i_low, q_low);
      byte |= low_nibble;

      idx += samples_per_symbol * 2;

      output.push(byte);
    }

    Ok(output)
  }

  fn name(&self) -> &str {
    "WAV Audio"
  }

  fn extension(&self) -> &str {
    "wav"
  }
}
