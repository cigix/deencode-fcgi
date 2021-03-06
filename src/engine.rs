//! Engines are objects that can analyze raw bytes.

use serde_json;

/// The common interface for engines.
pub trait Engine: Send + Sync {
  /// Parse a sequence of bytes into a String, or return a String describing the
  /// error.
  fn parse(&self, bytes: &Vec<u8>) -> Result<String, String>;

  /// Return a description string for every character in the string.
  fn describe(&self, string: &String) -> Vec<serde_json::Value>;

  fn get_name(&self) -> String;
}
