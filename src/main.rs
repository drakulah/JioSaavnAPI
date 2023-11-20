use std::path::PathBuf;

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
  let store = valkeyre::database::Database::init(PathBuf::from(".temp"), "valk");
  let table = store.init_table("t");

  match client
    .search("radha", 3, 20, SearchFilter::SongResults)
    .await
  {
    Ok(res) => {
      match JioSaavnResponseParser::parse_song_results(res) {
        Some(parsed) => {
          if let Ok(json_str) = serde_json::to_string(&parsed) {
            table.set("k", &json_str);
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
