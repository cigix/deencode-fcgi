use std::collections::HashMap;
use std::fs::File;

use serde_json;

pub type CodePoint = u32;

fn codepoint_from_str(s: &str) -> Result<u32, String>
{
  u32::from_str_radix(s, 16).map_err(|e| e.to_string())
}

/// An entry in the Unicode Database.
///
/// Note that the fields are not an exact representation of the actual Unicode
/// entry:
///   - some fields may require completion from other tables, i.e. Jamo
///   - some fields may be an amalgamate of information
///   - some fields may have to be combined to represent the information
/// Unless specified otherwise, the name of a field is the name of the Unicode
/// property.
pub struct UnicodeDatabaseEntry {
  pub codepoint: CodePoint,
  pub name: String,
  pub general_category: String,
  pub canonical_combining_class: u8,
  pub bidi_class: String,
  /// "<Decomposition_Type> Decomposition_Mapping"
  /// omitted if Decomposition_Mapping == codepoint
  pub decomposition: Option<String>,
  /// Numeric_Value
  /// omitted if not a decimal digit
  pub decimal_digit: Option<i32>,
  /// Numeric_Value
  /// omitted if not a digit
  pub digit: Option<i32>,
  /// Numeric_Value
  /// omitted if not a number
  // not sure what people are going to do with this information, better keep
  // it as string for now
  pub value: Option<String>,
  pub bidi_mirrored: bool,
  /// Unicode 1.0 or ISO 6429 name, for compatibility
  pub unicode_1_name: String,
  pub iso_comment: String,
  /// omitted if Simple_Uppercase_Mapping == codepoint
  pub simple_uppercase_mapping: Option<CodePoint>,
  /// omitted if Simple_Lowercase_Mapping == codepoint
  pub simple_lowercase_mapping: Option<CodePoint>,
  /// omitted if Simple_Titlecase_Mapping == codepoint
  pub simple_titlecase_mapping: Option<CodePoint>
}

type UnicodeDatabaseCsvRecord = (
  String, String, String, u8, String, Option<String>, Option<i32>, Option<i32>,
  Option<String>, char, String, String, Option<String>, Option<String>,
  Option<String>);

pub struct UnicodeDatabase(Vec<UnicodeDatabaseEntry>);

impl UnicodeDatabase {
  pub fn from(csv_reader: &mut csv::Reader<File>)
    -> Result<UnicodeDatabase, String>
  {
    let mut database: Vec<UnicodeDatabaseEntry> = Vec::new();
    for csvrecord in csv_reader.deserialize()
    {
      let record: UnicodeDatabaseCsvRecord = csvrecord
        .map_err(|e| e.to_string())?;
      let entry = UnicodeDatabaseEntry {
        codepoint: codepoint_from_str(record.0.as_str())?,
        name: record.1,
        general_category: record.2,
        canonical_combining_class: record.3,
        bidi_class: record.4,
        decomposition: record.5,
        decimal_digit: record.6,
        digit: record.7,
        value: record.8,
        bidi_mirrored: record.9 == 'Y',
        unicode_1_name: record.10,
        iso_comment: record.11,
        simple_uppercase_mapping:
          record.12.map(|c| codepoint_from_str(c.as_str())).transpose()?,
        simple_lowercase_mapping:
          record.13.map(|c| codepoint_from_str(c.as_str())).transpose()?,
        simple_titlecase_mapping:
          record.14.map(|c| codepoint_from_str(c.as_str())).transpose()?
      };
      database.push(entry);
    }

    Ok(UnicodeDatabase(database))
  }

  pub fn iter(&self) -> std::slice::Iter<UnicodeDatabaseEntry>
  {
    self.0.iter()
  }
}

pub struct UnicodeDataJson(HashMap<CodePoint, serde_json::Value>);

impl UnicodeDataJson {
  pub fn from(database: UnicodeDatabase) -> UnicodeDataJson
  {
    let mut map: HashMap<CodePoint, serde_json::Value> = HashMap::new();
    for entry in database.iter()
    {
      map.insert(entry.codepoint, serde_json::json!({
        "codepoint": format!("U+{:04X}", entry.codepoint),
        "name": if entry.name == "<control>" {
          format!("(control) {}", entry.unicode_1_name)
        } else {
          entry.name.clone()
        }
      }));
    }
    UnicodeDataJson(map)
  }

  pub fn get(&self, key: char) -> Option<serde_json::Value>
  {
    let codepoint: CodePoint = key as u32;
    let object = self.0.get(&codepoint)?;
    if let serde_json::Value::Object(mut map) = object.to_owned() {
      map.insert(String::from("character"), serde_json::json!(key));
      Some(serde_json::Value::Object(map))
    } else {
      panic!("Expected JSON object")
    }
  }
}
