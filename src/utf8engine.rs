use std::sync::Arc;

use crate::unicode;
use crate::engine;

pub struct Utf8Engine {
  unicode_data: Arc<unicode::UnicodeDatabase>
}

impl Utf8Engine {
  pub fn new(unicode_data: &Arc<unicode::UnicodeDatabase>) -> Utf8Engine
  {
    Utf8Engine { unicode_data: unicode_data.clone() }
  }
}

impl engine::Engine for Utf8Engine {
  fn parse(&self, bytes: &Vec<u8>) -> Result<String, String>
  {
    String::from_utf8(bytes.clone())
      .map_err(|e| format!("Could not convert byte sequence: {:?}\n",
                           e.into_bytes()))
  }

  fn describe(&self, string: &String) -> Vec<String>
  {
    let mut res: Vec<String> = Vec::with_capacity(string.len());
    for c in string.chars()
    {
      res.push(format!("{:2} U+{:04X} {}", c, c as u32,
                       self.unicode_data.get(c as u32).unwrap().name));
    }
    return res;
  }
}

unsafe impl Send for Utf8Engine {}
unsafe impl Sync for Utf8Engine {}
