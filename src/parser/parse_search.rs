use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{parse_song::JioSaavnSong, JioSaavnPartialParser, JioSaavnResponseParser, ValueExtras};

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnSearch {
  pub total: i64,
  pub start: i64,
  pub songs: Vec<JioSaavnSong>,
}

impl JioSaavnResponseParser {
  pub fn parse_song_results(text: String) -> Option<JioSaavnSearch> {
    match serde_json::from_str::<Value>(&text) {
      Ok(value) => {
        let mut results_array: Vec<JioSaavnSong> = Vec::new();
        let blank_array: &Vec<Value> = &Vec::new();

        let total = value["total"].get_int();
        let start = value["start"].get_int();

        /*** Ref ***/
        let results = &value["results"];

        for result in results.as_array().unwrap_or(blank_array).into_iter() {
          if let Some(parsed_song) = JioSaavnPartialParser::parse_song(result) {
            results_array.push(parsed_song);
          }
        }

        Some(JioSaavnSearch {
          total,
          start,
          songs: results_array,
        })
      }
      Err(_) => None,
    }
  }
}
