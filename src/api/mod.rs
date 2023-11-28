mod album;
mod artist;
mod autocomplete;
mod home;
mod playlist;
mod related_album;
mod related_playlist;
mod search;
mod song;
mod util;

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

pub struct JioSaavnConfig {
  pub lang: JioSaavnLanguage,
  pub music_lang: Vec<JioSaavnLanguage>,
}

pub struct JioSaavn {
  config: JioSaavnConfig,
}

impl JioSaavn {
  pub fn new(config: JioSaavnConfig) -> JioSaavn {
    JioSaavn { config }
  }
}
