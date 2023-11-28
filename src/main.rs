use std::path::PathBuf;

use api::{JioSaavn, JioSaavnConfig, JioSaavnLanguage};
use parser::JioSaavnResponseParser;

pub mod api;
pub mod client;
pub mod enums;
pub mod http;
pub mod parser;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let config = JioSaavnConfig {
    lang: JioSaavnLanguage::English,
    music_lang: vec![JioSaavnLanguage::English, JioSaavnLanguage::Hindi],
  };
  let client = JioSaavn::new(config);
  let store = valkeyre::store::Store::init(PathBuf::from(".temp"), "valk");
  let table = store.init_room("t");

  // match client.artist("LlRWpHzy3Hk_", 1, 20, 20, SortOrder::Asc, ArtistCategory::Alphabetical).await {
  //   Ok(res) => match JioSaavnResponseParser::parse_artist(res) {
  //     Some(parsed) => {
  //       if let Ok(json_str) = serde_json::to_string(&parsed) {
  //         table.set("l", &json_str);
  //       } else {
  //         println!("{:?}", parsed);
  //       }
  //     }
  //     None => {
  //       println!("None");
  //     }
  //   },
  //   Err(err) => {
  //     println!("{}", err.as_str());
  //   }
  // }

  match client.home().await {
    Ok(res) => {
      let parsed = JioSaavnResponseParser::parse_home(res);
      if let Ok(json_str) = serde_json::to_string(&parsed) {
        table.set("k", &json_str);
      } else {
        println!("{:?}", parsed);
      }
    }
    Err(err) => {
      println!("{}", err.as_str());
    }
  }

  Ok(())
}
