use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::parser::{extract_id_from_url, properize_explicit};

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

impl JioSaavnResponseParser {
  pub fn parse_playlist(text: String) -> Option<JioSaavnPlaylist> {
    match serde_json::from_str::<Value>(&text) {
      Ok(playlist) => {
        let more_info = &playlist["more_info"];

        let param = playlist["id"].get_string();
        let title = playlist["title"].get_string();
        let r#type = playlist["type"].get_string();
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

        Some(JioSaavnPlaylist {
          id,
          param,
          r#type,
          title,
          display_image,
          is_explicit,
          items_count,
          items,
          follower_count,
          fan_count,
          subtitles,
        })
      }
      Err(_) => None,
    }
  }
}
