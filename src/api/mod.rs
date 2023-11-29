use crate::enums::language::JioSaavnLanguage;

mod album;
mod artist;
mod auth_url;
mod autocomplete;
mod home;
mod playlist;
mod search;
mod song;
mod util;

pub struct JioSaavnConfig {
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
