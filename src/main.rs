use api::JioSaavn;
use enums::call::SearchFilter;
use parser::JioSaavnResponseParser;

pub mod api;
pub mod client;
pub mod enums;
pub mod http;
pub mod parser;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let client = JioSaavn::new();

  match client.search("Radha", 1, 10, SearchFilter::SongResults).await {
    Ok(res) => {
      match JioSaavnResponseParser::parse_song_results(res) {
        Some(parsed) => {
          if let Ok(json_str) = serde_json::to_string(&parsed) {
            println!("{}", json_str);
          } else {
            println!("{:?}", parsed);
          }
        }
        None => {
          println!("None");
        }
      }
      //println!("{}", par);
    }
    Err(err) => {
      println!("{}", err.as_str());
    }
  }

  Ok(())
}
