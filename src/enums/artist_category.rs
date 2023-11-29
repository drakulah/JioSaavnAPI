use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ArtistCategory {
  Latest,
  Popular,
  Alphabetical,
}

impl ArtistCategory {
  pub fn to_string(&self) -> String {
    match self {
      ArtistCategory::Popular => String::new(),
      ArtistCategory::Latest => String::from("latest"),
      ArtistCategory::Alphabetical => String::from("alphabetical"),
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      ArtistCategory::Popular => "",
      ArtistCategory::Latest => "latest",
      ArtistCategory::Alphabetical => "alphabetical",
    }
  }
}
