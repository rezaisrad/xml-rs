use std::io::BufReader;
use std::io::Read;
use std::io::{self, BufRead};
use error_chain::error_chain;
// use reqwest::blocking::Client;
// use reqwest::blocking::Request;
use reqwest::Method;
use reqwest::Url;
use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;
use std::str::FromStr;
use std::fs::File;
use quick_xml::reader::Reader;
use quick_xml::events::{Event, BytesStart};
use flate2::read::GzDecoder;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         Reqwest(reqwest::Error);
//         Header(reqwest::header::ToStrError);
//     }
// }

// struct PartialRangeIter {
//     start: u64,
//     end: u64,
//     buffer_size: u32,
//   }
  
//   impl PartialRangeIter {
//     pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
//       if buffer_size == 0 {
//         Err("invalid buffer_size, give a value greater than zero.")?;
//       }
//       Ok(PartialRangeIter {
//         start,
//         end,
//         buffer_size,
//       })
//     }
//   }
  
//   impl Iterator for PartialRangeIter {
//     type Item = HeaderValue;
//     fn next(&mut self) -> Option<Self::Item> {
//       if self.start > self.end {
//         None
//       } else {
//         let prev_start = self.start;
//         self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
//         Some(HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1)).expect("string provided by format!"))
//       }
//     }
//   }
  

// #[derive(Debug)]
// struct XMLInfo {
//     url: &'static str,
//     filename:  &'static str,
//     content_length: u64
// }

const URL: &str = "http://skoll.whatbox.ca:19869/sample_releases.xml";
const CHUNK_SIZE: u32 = 10240; // 10kb chunk-size 

error_chain! {
  foreign_links {
      Io(std::io::Error);
      HttpRequest(reqwest::Error);
  }
}

#[tokio::main]
async fn main() -> Result<()> {
 let response = reqwest::get(URL).await?;
 let content =  response.text().await?;
 
 let mut buf = Vec::new();
 let mut txt = Vec::new();
 let mut xml_reader = Reader::from_reader(content.as_bytes());
 loop {
      println!("Entered loop");
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Start(_)) => println!("Started new XML block"),
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
            Err(e) => panic!("Error at position {}: {:?}", xml_reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }

 Ok(())
}

// fn main() -> Result<()> {
//     // let url = Url::parse(&URL)
//     //                                     .expect("Failed to parse URL");
//     // let r = Request::new(Method::GET, url);

//     let client = Client::new();
//     println!("starting download...");
//     let response = client.head(URL,).send()?;
//     let status = response.status();
//     if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
//     error_chain::bail!("Unexpected server response: {}", status)
//     }
//     let length = response
//         .headers()
//         .get(CONTENT_LENGTH)
//         .ok_or("response doesn't include the content length")?;
//     let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?; 
//     let mut buf = Vec::new();

//     let bytes = response.read(&mut buf[..]);
//     // let mut gz = GzDecoder::new(cursor);
//     let mut txt = Vec::new();

//     // std::io::copy(&mut response, &mut output_file)?;
//     // let reader = gz.read(&mut buf).unwrap();
//     let mut xml_reader = Reader::from_reader(bytes);
//     xml_reader.trim_text(true);
//     loop {
//           println!("Entered loop");
//             match xml_reader.read(&mut buf) {
//                 Ok(Event::Start(_)) => println!("Started new XML block"),
//                 Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
//                 Err(e) => panic!("Error at position {}: {:?}", xml_reader.buffer_position(), e),
//                 Ok(Event::Eof) => break,
//                 _ => (),
//             }
//             buf.clear();
//         }
//     println!("completed download...");
//     println!("{:?}", &txt);
//     // for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
//     //     println!("range {:?}", range);
//     //     let response = client.get(URL).header(RANGE, range).send()?;
        
//     //     let status = response.status();
//     //     if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
//     //     error_chain::bail!("Unexpected server response: {}", status)
//     //     }
//     //     // std::io::copy(&mut response, &mut output_file)?;
        
//     //     let mut buf = Vec::new();
//     //     gz.read(&mut buf).unwrap();
        
//         // gz.read_buf(&buf)
//         // let mut xml_reader = Reader::from_reader();
//         // xml_reader.trim_text(true);
//         // loop {
//         //         match xml_reader.read_event_into(&mut buf) {
//         //             Ok(Event::Start(_)) => println!("Started new XML block"),
//         //             Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
//         //             Err(e) => panic!("Error at position {}: {:?}", xml_reader.buffer_position(), e),
//         //             Ok(Event::Eof) => break,
//         //             _ => (),
//         //         }
//         //         buf.clear();
//         //     }

//     // }

//     Ok(())

// }
