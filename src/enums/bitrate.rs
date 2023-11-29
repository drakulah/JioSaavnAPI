use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JioSaavnBitrate {
  Bitrate320,
  Bitrate256,
  Bitrate128,
}

impl JioSaavnBitrate {
  pub fn to_str(&self) -> &str {
    match self {
      JioSaavnBitrate::Bitrate320 => "320",
      JioSaavnBitrate::Bitrate256 => "256",
      JioSaavnBitrate::Bitrate128 => "128",
    }
  }
}
