use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{array::some_empty_string, parser::extract_id_from_url};

use super::{JioSaavnPartialParser, ValueExtras};

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnArtistBasicInfo {
  pub id: String,
  pub title: String,
  pub param: String,
  pub display_image: String,
}

impl JioSaavnPartialParser {
  pub fn parse_artist_basic_info(artist: &Value) -> Option<JioSaavnArtistBasicInfo> {
    let param = artist["id"].get_string().to_owned();
    let perma_url = artist["perma_url"].get_string();
    let title = artist["name"].get_string().to_owned();
    let image = artist["image"].get_string().to_owned();
    let id = extract_id_from_url(perma_url).to_owned();

    if !some_empty_string(&[id.clone(), title.clone(), param.clone()]) {
      Some(JioSaavnArtistBasicInfo {
        id,
        title,
        param,
        display_image: image,
      })
    } else {
      None
    }
  }
}
