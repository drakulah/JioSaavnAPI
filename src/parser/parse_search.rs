use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
  parse_album::JioSaavnAlbumPreview, parse_artist::JioSaavnArtistPreview,
  parse_playlist::JioSaavnPlaylistPreview, parse_song::JioSaavnSong, JioSaavnPartialParser,
  JioSaavnResponseParser, ValueExtras,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum JioSaavnSearchResultItemType {
  JioSaavnSong(JioSaavnSong),
  JioSaavnAlbumPreview(JioSaavnAlbumPreview),
  JioSaavnArtistPreview(JioSaavnArtistPreview),
  JioSaavnPlaylistPreview(JioSaavnPlaylistPreview),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnSearch {
  pub total: i64,
  pub start: i64,
  pub results: Vec<JioSaavnSearchResultItemType>,
}

impl JioSaavnResponseParser {
  pub fn parse_song_results(text: String) -> Option<JioSaavnSearch> {
    match serde_json::from_str::<Value>(&text) {
      Ok(value) => {
        let mut results: Vec<JioSaavnSearchResultItemType> = Vec::new();
        let blank_array: &Vec<Value> = &Vec::new();

        let total = value["total"].get_int();
        let start = value["start"].get_int();

        /*** Ref ***/
        let unparsed_results = &value["results"];

        for result in unparsed_results
          .as_array()
          .unwrap_or(blank_array)
          .into_iter()
        {
          if let Some(parsed_song) = JioSaavnPartialParser::parse_song(result) {
            results.push(JioSaavnSearchResultItemType::JioSaavnSong(parsed_song));
          }
        }

        for result in unparsed_results
          .as_array()
          .unwrap_or(blank_array)
          .into_iter()
        {
          if let Some(parsed_album_pre) = JioSaavnPartialParser::parse_album_preview(result) {
            results.push(JioSaavnSearchResultItemType::JioSaavnAlbumPreview(
              parsed_album_pre,
            ));
          }
        }

        for result in unparsed_results
          .as_array()
          .unwrap_or(blank_array)
          .into_iter()
        {
          if let Some(parsed_artist_pre) = JioSaavnPartialParser::parse_artist_preview(result) {
            results.push(JioSaavnSearchResultItemType::JioSaavnArtistPreview(
              parsed_artist_pre,
            ));
          }
        }

        for result in unparsed_results
          .as_array()
          .unwrap_or(blank_array)
          .into_iter()
        {
          if let Some(parsed_playlist_pre) = JioSaavnPartialParser::parse_playlist_preview(result) {
            results.push(JioSaavnSearchResultItemType::JioSaavnPlaylistPreview(
              parsed_playlist_pre,
            ));
          }
        }

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
