use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{
  array::some_empty_string,
  parser::{extract_id_from_url, properize_explicit},
};

use super::{
  parse_album::JioSaavnAlbumBasicInfo, parse_artist::JioSaavnArtistBasicInfo,
  JioSaavnPartialParser, JioSaavnResponseParser, ValueExtras,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnSong {
  pub id: String,
  pub r#type: String,
  pub title: String,
  pub lang: String,
  pub year: String,
  pub enc_media_url: String,
  pub display_image: String,
  pub plays: i64,
  pub duration: i64,
  pub is_explicit: bool,
  pub album: JioSaavnAlbumBasicInfo,
  pub artists: Vec<JioSaavnArtistBasicInfo>,
}

impl JioSaavnResponseParser {
  /**
   * [
   *    {
   *        "id": "5zJXFvhO",
   *        "title": "Different Colors",
   *        "type": "song",
   *        "perma_url": "https://www.jiosaavn.com/song/different-colors/RRIhaTJGX3w",
   *        "image": "http://c.saavncdn.com/689/TALKING-IS-HARD-English-2014-20200211230821-150x150.jpg",
   *        "language": "english",
   *        "year": "2014",
   *        "play_count": "6411",
   *        "explicit_content": "0",
   *        "more_info": {
   *            "album_id": "1206371",
   *            "album": "TALKING IS HARD",
   *            "encrypted_media_url": "iPPGVzyogeiPwpro65A0eUaQggN+8+J4aHbiPAIxpW3DetyR3quYNVxobKGg9vca6YJPL83a8Y0U6JDmDgHkt4PzFaL/aK97",
   *            "artistMap": {
   *                "primary_artists": [
   *                    {
   *                        "id": "577926",
   *                        "name": "WALK THE MOON",
   *                        "role": "primary_artists",
   *                        "image": "http://c.saavncdn.com/artists/WALK_THE_MOON_150x150.jpg",
   *                        "type": "artist",
   *                        "perma_url": "https://www.jiosaavn.com/artist/walk-the-moon-songs/e0q9qdHeIys_"
   *                    }
   *                ]
   *            }
   *        }
   *    }
   * ]
   */
  pub fn parse_song(text: String) -> Option<JioSaavnSong> {
    match serde_json::from_str::<Value>(&text) {
      Ok(value) => {
        /*** Ref ***/
        let song = &value["songs"][0];
        JioSaavnPartialParser::parse_song(song)
      }
      Err(_) => None,
    }
  }
}

impl JioSaavnPartialParser {
  /**
   * {
   *     "id": "5zJXFvhO",
   *     "title": "Different Colors",
   *     "type": "song",
   *     "perma_url": "https://www.jiosaavn.com/song/different-colors/RRIhaTJGX3w",
   *     "image": "http://c.saavncdn.com/689/TALKING-IS-HARD-English-2014-20200211230821-150x150.jpg",
   *     "language": "english",
   *     "year": "2014",
   *     "play_count": "6411",
   *     "explicit_content": "0",
   *     "more_info": {
   *         "album_id": "1206371",
   *         "album": "TALKING IS HARD",
   *         "encrypted_media_url": "iPPGVzyogeiPwpro65A0eUaQggN+8+J4aHbiPAIxpW3DetyR3quYNVxobKGg9vca6YJPL83a8Y0U6JDmDgHkt4PzFaL/aK97",
   *         "artistMap": {
   *             "primary_artists": [
   *                 {
   *                     "id": "577926",
   *                     "name": "WALK THE MOON",
   *                     "role": "primary_artists",
   *                     "image": "http://c.saavncdn.com/artists/WALK_THE_MOON_150x150.jpg",
   *                     "type": "artist",
   *                     "perma_url": "https://www.jiosaavn.com/artist/walk-the-moon-songs/e0q9qdHeIys_"
   *                 }
   *             ]
   *         }
   *     }
   * }
   */
  pub fn parse_song(song: &Value) -> Option<JioSaavnSong> {
    /*** Ref ***/
    let more_info = &song["more_info"];

    /*** Song ***/
    let id = extract_id_from_url(song["perma_url"].get_string());
    let title = song["title"].get_string();
    let r#type = song["type"].get_string();
    let lang = song["language"].get_string();
    let year = song["year"].get_string();
    let display_image = song["image"].get_string();
    let enc_media_url = more_info["encrypted_media_url"].get_string();
    let plays = song["play_count"].get_str_as_int();
    let duration = more_info["duration"].get_str_as_int();
    let is_explicit = properize_explicit(song["explicit_content"].get_string());

    /*** Validate Song ***/
    if some_empty_string(&[
      id.clone(),
      title.clone(),
      r#type.clone(),
      enc_media_url.clone(),
    ]) {
      return None;
    }

    if duration == -1 || plays == -1 {
      return None;
    }

    /*** Album & Artists ***/
    let nullable_album = JioSaavnPartialParser::parse_album_basic_info(more_info);
    let artist_array_default = &Vec::new();
    let artists_arr = more_info["artistMap"]["primary_artists"]
      .as_array()
      .unwrap_or(artist_array_default);
    let mut artists = Vec::new();

    for each_artist in artists_arr.into_iter() {
      if let Some(parsed_artist) = JioSaavnPartialParser::parse_artist_basic_info(each_artist) {
        artists.push(parsed_artist);
      }
    }

    /*** Validate Artists ***/
    if artists.is_empty() {
      return None;
    }

    match nullable_album {
      Some(album) => Some(JioSaavnSong {
        id,
        r#type: r#type.to_uppercase(),
        title,
        lang,
        year,
        enc_media_url,
        display_image,
        plays,
        duration,
        is_explicit,
        album,
        artists,
      }),
      None => None,
    }
  }
}
