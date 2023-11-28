use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
  parse_album::JioSaavnAlbumPreview, parse_artist::JioSaavnArtistPreview,
  parse_playlist::JioSaavnPlaylistPreview, parse_song::JioSaavnSong, JioSaavnPartialParser,
  JioSaavnResponseParser, ValueExtras,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum JioSaavnUnknownItemType {
  JioSaavnSong(JioSaavnSong),
  JioSaavnAlbumPreview(JioSaavnAlbumPreview),
  JioSaavnArtistPreview(JioSaavnArtistPreview),
  JioSaavnPlaylistPreview(JioSaavnPlaylistPreview),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnSearch {
  pub total: i64,
  pub start: i64,
  pub results: Vec<JioSaavnUnknownItemType>,
}

impl JioSaavnResponseParser {
  pub fn parse_song_results(text: String) -> Option<JioSaavnSearch> {
    match serde_json::from_str::<Value>(&text) {
      Ok(value) => {
        let total = value["total"].get_int();
        let start = value["start"].get_int();

        let results = JioSaavnPartialParser::parse_unknown_array(&value["results"]);

        Some(JioSaavnSearch {
          total,
          start,
          results,
        })
      }
      Err(_) => None,
    }
  }
}

impl JioSaavnPartialParser {
  pub fn parse_unknown_array(unknown_arr: &Value) -> Vec<JioSaavnUnknownItemType> {
    let mut results: Vec<JioSaavnUnknownItemType> = Vec::new();
    let blank_array: &Vec<Value> = &Vec::new();

    for result in unknown_arr.as_array().unwrap_or(blank_array).into_iter() {
      if let Some(parsed_song) = JioSaavnPartialParser::parse_song(result) {
        results.push(JioSaavnUnknownItemType::JioSaavnSong(parsed_song));
      } else if let Some(parsed_album_pre) = JioSaavnPartialParser::parse_album_preview(result) {
        results.push(JioSaavnUnknownItemType::JioSaavnAlbumPreview(
          parsed_album_pre,
        ));
      } else if let Some(parsed_artist_pre) = JioSaavnPartialParser::parse_artist_preview(result) {
        results.push(JioSaavnUnknownItemType::JioSaavnArtistPreview(
          parsed_artist_pre,
        ));
      } else if let Some(parsed_playlist_pre) =
        JioSaavnPartialParser::parse_playlist_preview(result)
      {
        results.push(JioSaavnUnknownItemType::JioSaavnPlaylistPreview(
          parsed_playlist_pre,
        ));
      }
    }

    results
  }
}
