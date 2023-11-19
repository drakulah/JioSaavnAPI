use serde_json::Value;

use crate::parser::{JioSaavnAlbumBasicInfo, JioSaavnArtistBasicInfo, ValueExtras};

use super::array::some_empty_string;

pub fn extract_id_from_url<S: AsRef<str>>(url: S) -> String {
  match url.as_ref().split("/").last() {
    Some(id) => id.to_owned(),
    None => String::new(),
  }
}

pub fn properize_explicit<S: AsRef<str>>(explicit: S) -> bool {
  match explicit.as_ref() {
    "1" => true,
    _ => false,
  }
}

pub fn parse_album_preview(val: &Value) -> Option<JioSaavnAlbumBasicInfo> {
  let url = val["album_url"].get_string();
  let id = extract_id_from_url(url).to_owned();
  let title = val["album"].get_string().to_owned();

  if some_empty_string(&[id.clone(), title.clone()]) {
    None
  } else {
    Some(JioSaavnAlbumBasicInfo { id, title })
  }
}

pub fn parse_artists_preview(artists: &Vec<Value>) -> Vec<JioSaavnArtistBasicInfo> {
  let mut parsed_artists: Vec<JioSaavnArtistBasicInfo> = Vec::new();
  for val in artists.into_iter() {
    let role = val["role"].get_string();
    let param = val["id"].get_string().to_owned();

    if role != "singer" || parsed_artists.iter().any(|e| e.param == param) {
      continue;
    }

    let perma_url = val["perma_url"].get_string();
    let title = val["name"].get_string().to_owned();
    let image = val["image"].get_string().to_owned();
    let id = extract_id_from_url(perma_url).to_owned();

    if !some_empty_string(&[id.clone(), title.clone(), image.clone(), param.clone()]) {
      parsed_artists.push(JioSaavnArtistBasicInfo {
        id,
        title,
        param,
        display_image: image,
      });
    }
  }
  parsed_artists
}
