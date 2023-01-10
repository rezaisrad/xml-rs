use std::env::args;
use std::io::Read;
use std::io::Cursor;
use error_chain::error_chain;
use quick_xml::reader::Reader;
use quick_xml::events::Event;
use flate2::read::GzDecoder;

error_chain! {
  foreign_links {
      Io(std::io::Error);
      HttpRequest(reqwest::Error);
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  let args: Vec<String> = args().collect();

  let url = &args[1];

  // Download response data as bytes
  println!("Begin Downloading file from: {}", url);
  let response = reqwest::get(url).await?.bytes().await?;
  println!("Completed Request: {} bytes", &response.len());
  //create vector to store buffer for gzip and for xml
  let mut buf = Vec::new();

  // Decompress .gz file and load into buffer
  println!("Begin Loading response to GZIP decoder");
  let mut gz = GzDecoder::new(&response[..]);
  gz.read_to_end(&mut buf)?;
  println!("Completed reading {} bytes into buffer", &buf.len());

  // Create reader var for XML data from buffer
  println!("Create Reader from Buffer");
  let mut reader = Reader::from_reader(Cursor::new(&mut buf));
  reader.trim_text(true);

  // Iterate through each xml event
  let mut txt = Vec::new();
  println!("Begin iterating through XML events");
  loop {
          match reader.read_event_into(&mut txt) {
              Ok(Event::Start(e)) => println!("{:?} Tag", e.name().decompose().0),
              Ok(Event::Text(e)) => println!("{} Text", e.unescape().unwrap()),
              Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
              Ok(Event::Eof) => break,
              _ => (),
          }
          txt.clear(); 
      }
  println!("Completed iterating through XML events");
  Ok(())
}