use crate::api::JioSaavn;

use super::JioSaavnLanguage;

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

impl JioSaavn {
  pub fn config_as_cookie(&self) -> String {
    let lang = &self.config.lang;
    let music_lang = &self.config.music_lang;
    let mut music_lang_str = Vec::new();

    for l in music_lang.into_iter() {
      music_lang_str.push(l.to_str());
    }

    format!("DL={}; L={}", lang.to_str(), music_lang_str.join("%2C"))
  }
}
