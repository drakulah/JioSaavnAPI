use crate::api::JioSaavn;

impl JioSaavn {
  pub fn config_as_cookie(&self) -> String {
    let music_lang = &self.config.music_lang;
    let mut music_lang_str = Vec::new();

    for l in music_lang.into_iter() {
      music_lang_str.push(l.to_str());
    }

    format!("DL=english; L={}", music_lang_str.join("%2C"))
  }
}
