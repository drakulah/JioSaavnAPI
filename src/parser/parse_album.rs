use serde_json::Value;

use crate::{
  types::{JioSaavnAlbum, JioSaavnAlbumBasicInfo, JioSaavnAlbumPreview},
  utils::{
    array::some_empty_string,
    parser::{extract_id_from_url, properize_explicit},
  },
};

use super::{JioSaavnPartialParser, JioSaavnResponseParser, ValueExtras};

impl JioSaavnResponseParser {
  pub fn parse_album(text: String) -> Option<JioSaavnAlbum> {
    match serde_json::from_str::<Value>(&text) {
      Ok(album) => {
        let more_info = &album["more_info"];

        let url = album["perma_url"].get_string();
        let id = extract_id_from_url(url);
        let param = album["id"].get_string();
        let lang = album["language"].get_string();
        let year = album["year"].get_string();
        let r#type = album["type"].get_string().to_uppercase();
        let title = album["title"].get_string();
        let display_image = album["image"].get_string();
        let is_explicit = properize_explicit(album["explicit_content"].get_string());

        let artist_array_default = &Vec::new();
        let artists_arr = more_info["artistMap"]["artists"]
          .as_array()
          .unwrap_or(artist_array_default);
        let mut artists = Vec::new();

        for each_artist in artists_arr.into_iter() {
          if let Some(parsed_artist) = JioSaavnPartialParser::parse_artist_preview(each_artist) {
            artists.push(parsed_artist);
          }
        }

        let items_array_default = &Vec::new();
        let items_arr = album["list"].as_array().unwrap_or(items_array_default);
        let mut items = Vec::new();

        for each_item in items_arr.into_iter() {
          if let Some(parsed_song) = JioSaavnPartialParser::parse_song(each_item) {
            items.push(parsed_song);
          }
        }

        if some_empty_string(&[id.clone(), title.clone()]) || r#type != "ALBUM" {
          return None;
        }

        Some(JioSaavnAlbum {
          id,
          lang,
          year,
          param,
          r#type: r#type.to_uppercase(),
          title,
          is_explicit,
          display_image,
          artists,
          items,
        })
      }
      Err(_) => None,
    }
  }

  pub fn parse_related_albums(text: String) -> Vec<JioSaavnAlbumPreview> {
    let mut album_pre_array: Vec<JioSaavnAlbumPreview> = Vec::new();
    match serde_json::from_str::<Value>(&text) {
      Ok(value) => {
        let blank_array: &Vec<Value> = &Vec::new();

        for each_album_preview in value.as_array().unwrap_or(blank_array).into_iter() {
          if let Some(parsed_album_pre) =
            JioSaavnPartialParser::parse_album_preview(each_album_preview)
          {
            album_pre_array.push(parsed_album_pre);
          }
        }
        album_pre_array
      }
      Err(_) => album_pre_array,
    }
  }
}

impl JioSaavnPartialParser {
  pub fn parse_album_basic_info(basic_album: &Value) -> Option<JioSaavnAlbumBasicInfo> {
    let url = basic_album["album_url"].get_string();
    let id = extract_id_from_url(url);
    let title = basic_album["album"].get_string();
    let param = basic_album["album_id"].get_string();

    if some_empty_string(&[id.clone(), title.clone(), param.clone()]) {
      return None;
    }

    Some(JioSaavnAlbumBasicInfo { id, title, param })
  }

  pub fn parse_album_preview(album_preview: &Value) -> Option<JioSaavnAlbumPreview> {
    let url = album_preview["perma_url"].get_string();
    let id = extract_id_from_url(url);
    let r#type = album_preview["type"].get_string().to_uppercase();
    let title = album_preview["title"].get_string();
    let display_image = album_preview["image"].get_string();
    let param = album_preview["id"].get_string();
    let is_explicit = properize_explicit(album_preview["explicit_content"].get_string());

    let artist_array_default = &Vec::new();
    let artists_arr = album_preview["more_info"]["artistMap"]["artists"]
      .as_array()
      .unwrap_or(artist_array_default);
    let mut artists = Vec::new();

    for each_artist in artists_arr.into_iter() {
      if let Some(parsed_artist) = JioSaavnPartialParser::parse_artist_preview(each_artist) {
        artists.push(parsed_artist);
      }
    }

    if some_empty_string(&[id.clone(), title.clone(), param.clone()]) || r#type != "ALBUM" {
      return None;
    }

    Some(JioSaavnAlbumPreview {
      id,
      param,
      title,
      artists,
      is_explicit,
      display_image,
      r#type: "ALBUM_PREVIEW".to_string(),
    })
  }
}
