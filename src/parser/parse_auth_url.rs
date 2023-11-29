use std::str::FromStr;

use hyper::Uri;
use serde_json::Value;

use crate::types::JioSaavnAuthUrl;

use super::{JioSaavnResponseParser, ValueExtras};

impl JioSaavnResponseParser {
  pub fn parse_auth_url(text: String) -> Option<JioSaavnAuthUrl> {
    match serde_json::from_str::<Value>(&text) {
      Ok(value) => {
        let auth_url = value["auth_url"].get_string();
        let audio_format = value["type"].get_string();

        if audio_format.is_empty() || auth_url.is_empty() {
          return None;
        }

        if let Ok(uri) = Uri::from_str(&auth_url) {
          return Some(JioSaavnAuthUrl {
            stream_url: uri.to_string(),
            audio_format,
          });
        }

        None
      }
      Err(_) => None,
    }
  }
}
