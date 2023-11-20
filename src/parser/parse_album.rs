use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{array::some_empty_string, parser::extract_id_from_url};

use super::{JioSaavnPartialParser, ValueExtras};

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnAlbumBasicInfo {
  pub id: String,
  pub title: String,
}

impl JioSaavnPartialParser {
  pub fn parse_album_basic_info(basic_album: &Value) -> Option<JioSaavnAlbumBasicInfo> {
    let url = basic_album["album_url"].get_string();
    let id = extract_id_from_url(url).to_owned();
    let title = basic_album["album"].get_string().to_owned();

    if some_empty_string(&[id.clone(), title.clone()]) {
      None
    } else {
      Some(JioSaavnAlbumBasicInfo { id, title })
    }
  }
}
