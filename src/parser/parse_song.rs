use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{
  array::some_empty_string,
  parser::{extract_id_from_url, parse_album_preview, parse_artists_preview, properize_explicit},
};

use super::{
  JioSaavnAlbumBasicInfo, JioSaavnArtistBasicInfo, JioSaavnPartialParser, JioSaavnResponseParser,
  ValueExtras,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnSong {
  pub id: String,
  pub r#type: String,
  pub title: String,
  pub lang: String,
  pub year: String,
  pub enc_media_url: String,
  pub display_image: String,
  pub plays: i64,
  pub duration: i64,
  pub is_explicit: bool,
  pub album: JioSaavnAlbumBasicInfo,
  pub artists: Vec<JioSaavnArtistBasicInfo>,
}

impl JioSaavnResponseParser {
  pub fn parse_song(text: String) -> Option<JioSaavnSong> {
    match serde_json::from_str::<Value>(&text) {
      Ok(value) => {
        /*** Ref ***/
        let song = &value["songs"][0];
        JioSaavnPartialParser::parse_song(song.to_string())
      }
      Err(_) => None,
    }
  }
}

impl JioSaavnPartialParser {
  pub fn parse_song(text: String) -> Option<JioSaavnSong> {
    match serde_json::from_str::<Value>(&text) {
      Ok(song) => {
        /*** Ref ***/
        let more_info = &song["more_info"];

        /*** Song ***/
        let id = extract_id_from_url(song["perma_url"].get_string());
        let title = song["title"].get_string();
        let r#type = song["type"].get_string();
        let lang = song["language"].get_string();
        let year = song["year"].get_string();
        let display_image = song["image"].get_string();
        let enc_media_url = more_info["encrypted_media_url"].get_string();
        let plays = song["play_count"].get_str_as_int();
        let duration = more_info["duration"].get_str_as_int();
        let is_explicit = properize_explicit(song["explicit_content"].get_string());

        /*** Validate Song ***/
        if some_empty_string(&[
          id.clone(),
          title.clone(),
          r#type.clone(),
          lang.clone(),
          year.clone(),
          display_image.clone(),
          enc_media_url.clone(),
        ]) {
          return None;
        }

        if duration == -1 || plays == -1 {
          return None;
        }

        /*** Album & Artists ***/
        let nullable_album = parse_album_preview(more_info);
        let artist_array_default = &Vec::new();
        let artists_arr = more_info["artistMap"]["artists"]
          .as_array()
          .unwrap_or(artist_array_default);
        let artists = parse_artists_preview(artists_arr);

        /*** Validate Artists ***/
        if artists.is_empty() {
          return None;
        }

        match nullable_album {
          Some(album) => Some(JioSaavnSong {
            id,
            r#type,
            title,
            lang,
            year,
            enc_media_url,
            display_image,
            plays,
            duration,
            is_explicit,
            album,
            artists,
          }),
          None => None,
        }
      }
      Err(_) => None,
    }
  }
}
