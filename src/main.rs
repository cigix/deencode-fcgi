extern crate fastcgi;

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

  let mut results: Vec<String> = Vec::new();

  for e in engines
  {
    match e.parse(&buffer)
    {
      Ok(string) => {
        let mut descriptions = e.describe(&string);
        results.push(string);
        results.append(&mut descriptions);
      }
      Err(e) => { results.push(e); }
    }
  }

  return results.join("\n");
}

fn main()
{
  let mut csv_reader = csv::ReaderBuilder::new()
    .delimiter(b';')
    .has_headers(false)
    .from_path("UnicodeData.txt").unwrap();
  let unicode_data = Arc::new(
    unicode::UnicodeDatabase::from(&mut csv_reader)
    .unwrap() // Fatal error
    );

  let utf8 = utf8engine::Utf8Engine::new(&unicode_data);
  let engines: Vec<Box<dyn engine::Engine>> = vec![Box::new(utf8)];

  let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
  println!("Running on {:?}", listener);
  println!("Listening on {}", listener.local_addr().unwrap());

  fastcgi::run_tcp(move |mut request: fastcgi::Request| {
    println!("Got request");
    let response = handle_request(&mut request, &engines);
    write!(&mut request.stdout(),
           "Content-Type: text/plain; charset=utf-8\n\n{}\n", response)
      .unwrap(); // Fatal error
  }, &listener);
}
