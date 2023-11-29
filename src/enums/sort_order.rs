use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortOrder {
  Asc,
  Desc,
}

impl SortOrder {
  pub fn to_string(&self) -> String {
    match self {
      SortOrder::Asc => String::from("asc"),
      SortOrder::Desc => String::from("desc"),
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      SortOrder::Asc => "asc",
      SortOrder::Desc => "desc",
    }
  }
}
