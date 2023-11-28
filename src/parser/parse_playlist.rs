use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{
  array::some_empty_string,
  parser::{extract_id_from_url, properize_explicit},
};

use super::{parse_song::JioSaavnSong, JioSaavnPartialParser, JioSaavnResponseParser, ValueExtras};

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnPlaylist {
  pub id: String,
  pub param: String,
  pub r#type: String,
  pub title: String,
  pub display_image: String,
  pub is_explicit: bool,
  pub items_count: i64,
  pub items: Vec<JioSaavnSong>,
  pub follower_count: i64,
  pub fan_count: i64,
  pub subtitles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnPlaylistPreview {
  pub id: String,
  pub param: String,
  pub title: String,
  pub subtitle: String,
  pub r#type: String,
  pub display_image: String,
  pub is_explicit: bool,
  pub items_count: i64,
}

impl JioSaavnResponseParser {
  pub fn parse_playlist(text: String) -> Option<JioSaavnPlaylist> {
    match serde_json::from_str::<Value>(&text) {
      Ok(playlist) => {
        let more_info = &playlist["more_info"];

        let param = playlist["id"].get_string();
        let title = playlist["title"].get_string();
        let r#type = playlist["type"].get_string().to_uppercase();
        let url = playlist["perma_url"].get_string();
        let id = extract_id_from_url(url);
        let display_image = playlist["image"].get_string();
        let is_explicit = properize_explicit(playlist["explicit_content"].get_string());
        let items_count = playlist["list_count"].get_str_as_int();

        let fan_count = more_info["fan_count"].get_str_as_int();
        let follower_count = more_info["follower_count"].get_str_as_int();

        let mut subtitles: Vec<String> = Vec::new();

        for text_ref in more_info["subtitle_desc"].get_arr().into_iter() {
          let text = text_ref.get_string();
          if !text.is_empty() {
            subtitles.push(text);
          }
        }

        let mut items: Vec<JioSaavnSong> = Vec::new();

        for item_ref in playlist["list"].get_arr().into_iter() {
          if let Some(item) = JioSaavnPartialParser::parse_song(&item_ref) {
            items.push(item);
          }
        }

        if some_empty_string(&[id.clone(), title.clone(), param.clone()]) || r#type != "PLAYLIST" {
          return None;
        }

        Some(JioSaavnPlaylist {
          id,
          param,
          r#type,
          title,
          subtitles,
          display_image,
          is_explicit,
          items_count,
          items,
          follower_count,
          fan_count,
        })
      }
      Err(_) => None,
    }
  }

  pub fn parse_related_playlist(text: String) -> Vec<JioSaavnPlaylistPreview> {
    let mut playlist_pre_arr: Vec<JioSaavnPlaylistPreview> = Vec::new();
    match serde_json::from_str::<Value>(&text) {
      Ok(v) => {
        for maybe_playlist_pre in v.get_arr().into_iter() {
          if let Some(playlist_pre) =
            JioSaavnPartialParser::parse_playlist_preview(&maybe_playlist_pre)
          {
            playlist_pre_arr.push(playlist_pre);
          }
        }

        playlist_pre_arr
      }
      Err(_) => playlist_pre_arr,
    }
  }
}

impl JioSaavnPartialParser {
  pub fn parse_playlist_preview(playlist_pre: &Value) -> Option<JioSaavnPlaylistPreview> {
    let param = playlist_pre["id"].get_string();
    let title = playlist_pre["title"].get_string();
    let url = playlist_pre["perma_url"].get_string();
    let r#type = playlist_pre["type"].get_string().to_uppercase();
    let id = extract_id_from_url(url);
    let subtitle = playlist_pre["subtitle"].get_string();
    let display_image = playlist_pre["image"].get_string();
    let items_count = playlist_pre["more_info"]["song_count"].get_str_as_int();
    let is_explicit = properize_explicit(playlist_pre["explicit_content"].get_string());

    if some_empty_string(&[id.clone(), title.clone(), param.clone()]) || r#type != "PLAYLIST" {
      return None;
    }

    Some(JioSaavnPlaylistPreview {
      id,
      param,
      title,
      subtitle,
      r#type: "PLAYLIST_PREVIEW".to_string(),
      display_image,
      is_explicit,
      items_count,
    })
  }
}
