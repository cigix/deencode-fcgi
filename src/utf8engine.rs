use std::sync::Arc;

use serde_json;

use crate::unicode;
use crate::engine;

pub struct Utf8Engine {
  unicode_data_json: Arc<unicode::UnicodeDataJson>
}

impl Utf8Engine {
  pub fn new(unicode_data_json: &Arc<unicode::UnicodeDataJson>) -> Utf8Engine
  {
    Utf8Engine { unicode_data_json: unicode_data_json.clone() }
  }
}

impl engine::Engine for Utf8Engine {
  fn parse(&self, bytes: &Vec<u8>) -> Result<String, String>
  {
    String::from_utf8(bytes.clone())
      .map_err(|e| format!("Could not convert byte sequence: {:?}\n",
                           e.into_bytes()))
  }

  fn describe(&self, string: &String) -> Vec<serde_json::Value>
  {
    string.chars()
      .map(|c| self.unicode_data_json.get(c).unwrap())
      .collect()
  }

  fn get_name(&self) -> String
  {
    String::from("UTF-8")
  }
}

unsafe impl Send for Utf8Engine {}
unsafe impl Sync for Utf8Engine {}
