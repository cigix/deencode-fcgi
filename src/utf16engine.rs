use std::sync::Arc;

use serde_json;

use crate::unicode;
use crate::engine;

// Explanation about BOM:
// It is necessary to know the endianness of the data, i.e. if 0x1234 will be
// read in the order 0x12 0x34 or 0x34 0x12. The character U+FEFF ZERO WIDTH
// NO-BREAK SPACE is often put as the first character in the data: if it is read
// as 0xFEFF, the endianness is good, but if 0xFFEF is read (which is not a
// valid Unicode scalar value), then it means we have the wrong endianness.

pub struct Utf16BEEngine {
  unicode_data_json: Arc<unicode::UnicodeDataJson>
}
pub struct Utf16LEEngine {
  unicode_data_json: Arc<unicode::UnicodeDataJson>
}

impl Utf16BEEngine {
  pub fn new(unicode_data_json: &Arc<unicode::UnicodeDataJson>) -> Utf16BEEngine
  {
    Utf16BEEngine { unicode_data_json: unicode_data_json.clone() }
  }
}
impl Utf16LEEngine {
  pub fn new(unicode_data_json: &Arc<unicode::UnicodeDataJson>) -> Utf16LEEngine
  {
    Utf16LEEngine { unicode_data_json: unicode_data_json.clone() }
  }
}

impl engine::Engine for Utf16BEEngine {
  fn parse(&self, bytes: &Vec<u8>) -> Result<String, String>
  {
    if bytes.len() % 2 != 0 {
      return Err(String::from("Not an even amount of bytes"));
    }

    let words: Vec<u16> = bytes
      .chunks(2)
      .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
      .collect();

    if words.get(0) == Some(&0xFFEF) {
      return Err(String::from("BOM in wrong order, see Utf16LEEngine"))
    }

    String::from_utf16(words.as_slice())
      .map_err(|e| format!("Could not convert word sequence: {}\n", e))
  }

  fn describe(&self, string: &String) -> Vec<serde_json::Value>
  {
    string.chars()
      .map(|c| self.unicode_data_json.describe(c))
      .collect()
  }

  fn get_name(&self) -> String
  {
    String::from("UTF-16 Big Endian")
  }
}
impl engine::Engine for Utf16LEEngine {
  fn parse(&self, bytes: &Vec<u8>) -> Result<String, String>
  {
    if bytes.len() % 2 != 0 {
      return Err(String::from("Not an even amount of bytes"));
    }

    let words: Vec<u16> = bytes
      .chunks(2)
      .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
      .collect();

    if words.get(0) == Some(&0xFFEF) {
      return Err(String::from("BOM in wrong order, see Utf16BEEngine"))
    }

    String::from_utf16(words.as_slice())
      .map_err(|e| format!("Could not convert word sequence: {}\n", e))
  }

  fn describe(&self, string: &String) -> Vec<serde_json::Value>
  {
    string.chars()
      .map(|c| self.unicode_data_json.describe(c))
      .collect()
  }

  fn get_name(&self) -> String
  {
    String::from("UTF-16 Little Endian")
  }
}

unsafe impl Send for Utf16BEEngine {}
unsafe impl Send for Utf16LEEngine {}
unsafe impl Sync for Utf16BEEngine {}
unsafe impl Sync for Utf16LEEngine {}
