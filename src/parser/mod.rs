use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::entity;

mod parse_search;
mod parse_song;
mod parse_artist;
mod parse_album;

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnPartialParser {}

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnResponseParser {}

pub trait ValueExtras {
  fn get_string(&self) -> String;
  fn get_int(&self) -> i64;
  fn get_str_as_int(&self) -> i64;
}

impl ValueExtras for Value {
  fn get_string(&self) -> String {
    let got = self.as_str().unwrap_or("");
    entity::decode(got).to_string()
  }

  fn get_int(&self) -> i64 {
    self.as_i64().unwrap_or(-1)
  }

  fn get_str_as_int(&self) -> i64 {
    match self.as_str().unwrap_or("").parse::<i64>() {
      Ok(integer) => integer,
      Err(_) => -1,
    }
  }
}
