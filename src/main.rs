use std::io::Read;
use std::io::Cursor;
use error_chain::error_chain;
use quick_xml::reader::Reader;
use quick_xml::events::Event;
use flate2::read::GzDecoder;
// use reqwest::blocking::Client;
// use reqwest::blocking::Request;

// #[derive(Debug)]
// struct XMLInfo {
//     url: &'static str,
//     filename:  &'static str,
//     content_length: u64
// }

const URL: &str = "http://skoll.whatbox.ca:19869/sample_releases.xml.gz";
// const CHUNK_SIZE: u32 = 10240; // 10kb chunk-size 

error_chain! {
  foreign_links {
      Io(std::io::Error);
      HttpRequest(reqwest::Error);
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  // get response data as bytes
  let response = reqwest::get(URL).await?.bytes().await?;

  // create two vectors to store buffer for gzip and for xml
  let mut txt = Vec::new();
  let mut buf = Vec::new();

  // Decode gz file and load into buffer
  let mut gz = GzDecoder::new(&response[..]);
  gz.read_to_end(&mut buf)?;

  // Create reader var for XML data from buffer
  let mut reader = Reader::from_reader(Cursor::new(&mut buf));
  reader.trim_text(true);

  // Iterate through each xml event
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

 Ok(())
}