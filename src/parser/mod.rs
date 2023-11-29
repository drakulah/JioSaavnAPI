use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::entity;

pub mod parse_album;
pub mod parse_artist;
pub mod parse_auth_url;
pub mod parse_home;
pub mod parse_playlist;
pub mod parse_search;
pub mod parse_song;

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnPartialParser {}

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnResponseParser {}

pub trait ValueExtras {
  fn get_int(&self) -> i64;
  fn get_string(&self) -> String;
  fn get_str_as_int(&self) -> i64;
  fn get_arr(&self) -> Vec<Value>;
}

impl ValueExtras for Value {
  fn get_string(&self) -> String {
    entity::decode(self.as_str().unwrap_or("")).to_string()
  }

  fn get_int(&self) -> i64 {
    self.as_i64().unwrap_or(0)
  }

  fn get_str_as_int(&self) -> i64 {
    match self.as_str().unwrap_or("").replace(",", "").parse::<i64>() {
      Ok(integer) => integer,
      Err(_) => 0,
    }
  }

  fn get_arr(&self) -> Vec<Value> {
    let def = &Vec::new();
    self.as_array().unwrap_or(def).to_vec()
  }
}
