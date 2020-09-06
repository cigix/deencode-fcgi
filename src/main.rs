extern crate fastcgi;
extern crate serde_json;

use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::sync::Arc;

mod engine;
mod unicode;
mod utf8engine;

fn handle_request(request: &mut fastcgi::Request,
                  engines: &Vec<Box<dyn engine::Engine>>) -> String
{
  let mut buffer: Vec<u8> = Vec::new();
  request.stdin().read_to_end(&mut buffer).unwrap(); // Fatal error

  let mut results = serde_json::map::Map::<String, serde_json::Value>::new();

  for e in engines
  {
    results.insert(e.get_name(), match e.parse(&buffer)
    {
      Ok(string) =>
        serde_json::json!({
          "parsed": string,
          "description": e.describe(&string)
        }),
      Err(error) => serde_json::json!({
        "error": error
      })
    });
  }

  return serde_json::Value::Object(results).to_string();
}

fn main()
{
  let mut csv_reader = csv::ReaderBuilder::new()
    .delimiter(b';')
    .has_headers(false)
    .from_path("UnicodeData.txt").unwrap();
  let unicode_database = unicode::UnicodeDatabase::from(&mut csv_reader)
    .unwrap(); // Fatal error
  let unicode_data_json = Arc::new(
    unicode::UnicodeDataJson::from(unicode_database));

  let utf8 = utf8engine::Utf8Engine::new(&unicode_data_json);
  let engines: Vec<Box<dyn engine::Engine>> = vec![Box::new(utf8)];

  let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
  println!("Running on {:?}", listener);
  println!("Listening on {}", listener.local_addr().unwrap());

  fastcgi::run_tcp(move |mut request: fastcgi::Request| {
    println!("Got request");
    let response = handle_request(&mut request, &engines);
    write!(&mut request.stdout(),
           "Content-Type: application/json; charset=utf-8\n\n{}\n", response)
      .unwrap(); // Fatal error
  }, &listener);
}
