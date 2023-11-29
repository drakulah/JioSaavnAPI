use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JioSaavnLanguage {
  Hindi,
  English,
  Punjabi,
  Tamil,
  Telugu,
  Marathi,
  Gujarati,
  Bengali,
  Kannada,
  Bhojpuri,
  Malayalam,
  Urdu,
  Haryanvi,
  Rajasthani,
  Odia,
  Assamese,
}

impl JioSaavnLanguage {
  pub fn to_str(&self) -> &str {
    match self {
      JioSaavnLanguage::Hindi => "hindi",
      JioSaavnLanguage::English => "english",
      JioSaavnLanguage::Punjabi => "punjabi",
      JioSaavnLanguage::Tamil => "tamil",
      JioSaavnLanguage::Telugu => "telugu",
      JioSaavnLanguage::Marathi => "marathi",
      JioSaavnLanguage::Gujarati => "gujarati",
      JioSaavnLanguage::Bengali => "bengali",
      JioSaavnLanguage::Kannada => "kannada",
      JioSaavnLanguage::Bhojpuri => "bhojpuri",
      JioSaavnLanguage::Malayalam => "malayalam",
      JioSaavnLanguage::Urdu => "urdu",
      JioSaavnLanguage::Haryanvi => "haryanvi",
      JioSaavnLanguage::Rajasthani => "rajasthani",
      JioSaavnLanguage::Odia => "odia",
      JioSaavnLanguage::Assamese => "assamese",
    }
  }
}
