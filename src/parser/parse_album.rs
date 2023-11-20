use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{
  array::some_empty_string,
  parser::{extract_id_from_url, properize_explicit},
};

use super::{
  parse_artist::JioSaavnArtistBasicInfo, parse_song::JioSaavnSong, JioSaavnPartialParser,
  JioSaavnResponseParser, ValueExtras,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnAlbumBasicInfo {
  pub id: String,
  pub param: String,
  pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnAlbumPreview {
  pub id: String,
  pub param: String,
  pub r#type: String,
  pub title: String,
  pub is_explicit: bool,
  pub display_image: String,
  pub artists: Vec<JioSaavnArtistBasicInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JioSaavnAlbum {
  pub id: String,
  pub param: String,
  pub r#type: String,
  pub title: String,
  pub lang: String,
  pub year: String,
  pub is_explicit: bool,
  pub display_image: String,
  pub artists: Vec<JioSaavnArtistBasicInfo>,
  pub items: Vec<JioSaavnSong>,
}

impl JioSaavnResponseParser {
  /**
   * {
   *    "id": "1206371",
   *    "title": "TALKING IS HARD",
   *    "type": "album",
   *    "perma_url": "https://www.jiosaavn.com/album/talking-is-hard/9yl-XupdHyI_",
   *    "image": "http://c.saavncdn.com/689/TALKING-IS-HARD-English-2014-20200211230821-150x150.jpg",
   *    "language": "english",
   *    "year": "2014",
   *    "explicit_content": "0",
   *    "list_count": "12",
   *    "list_type": "song",
   *    "list": [
   *         {
   *             "id": "5zJXFvhO",
   *             "title": "Different Colors",
   *             "type": "song",
   *             "perma_url": "https://www.jiosaavn.com/song/different-colors/RRIhaTJGX3w",
   *             "image": "http://c.saavncdn.com/689/TALKING-IS-HARD-English-2014-20200211230821-150x150.jpg",
   *             "language": "english",
   *             "year": "2014",
   *             "play_count": "6411",
   *             "explicit_content": "0",
   *             "more_info": {
   *                 "album_id": "1206371",
   *                 "album": "TALKING IS HARD",
   *                 "encrypted_media_url": "iPPGVzyogeiPwpro65A0eUaQggN+8+J4aHbiPAIxpW3DetyR3quYNVxobKGg9vca6YJPL83a8Y0U6JDmDgHkt4PzFaL/aK97",
   *                 "artistMap": {
   *                     "primary_artists": [
   *                         {
   *                             "id": "577926",
   *                             "name": "WALK THE MOON",
   *                             "role": "primary_artists",
   *                             "image": "http://c.saavncdn.com/artists/WALK_THE_MOON_150x150.jpg",
   *                             "type": "artist",
   *                             "perma_url": "https://www.jiosaavn.com/artist/walk-the-moon-songs/e0q9qdHeIys_"
   *                         }
   *                     ]
   *                 }
   *             }
   *         }
   *    ],
   *    "more_info": {
   *        "artistMap": {
   *            "artists": [
   *                {
   *                    "id": "577926",
   *                    "name": "WALK THE MOON",
   *                    "role": "",
   *                    "image": "http://c.saavncdn.com/artists/WALK_THE_MOON_150x150.jpg",
   *                    "type": "artist",
   *                    "perma_url": "https://www.jiosaavn.com/artist/walk-the-moon-songs/e0q9qdHeIys_"
   *                }
   *            ]
   *        }
   *    }
   * }
   */
  pub fn parse_album(text: String) -> Option<JioSaavnAlbum> {
    match serde_json::from_str::<Value>(&text) {
      Ok(album) => {
        /*** Ref ***/
        let more_info = &album["more_info"];

        let url = album["perma_url"].get_string();
        let id = extract_id_from_url(url);
        let param = album["id"].get_string();
        let lang = album["language"].get_string();
        let year = album["year"].get_string();
        let r#type = album["type"].get_string();
        let title = album["title"].get_string();
        let display_image = album["image"].get_string();
        let is_explicit = properize_explicit(album["explicit_content"].get_string());

        let artist_array_default = &Vec::new();
        let artists_arr = more_info["artistMap"]["artists"]
          .as_array()
          .unwrap_or(artist_array_default);
        let mut artists = Vec::new();

        for each_artist in artists_arr.into_iter() {
          if let Some(parsed_artist) = JioSaavnPartialParser::parse_artist_basic_info(each_artist) {
            artists.push(parsed_artist);
          }
        }

        let items_array_default = &Vec::new();
        let items_arr = album["list"].as_array().unwrap_or(items_array_default);
        let mut items = Vec::new();

        for each_item in items_arr.into_iter() {
          if let Some(parsed_song) = JioSaavnPartialParser::parse_song(each_item) {
            items.push(parsed_song);
          }
        }

        if some_empty_string(&[id.clone(), title.clone()]) {
          None
        } else {
          Some(JioSaavnAlbum {
            id,
            lang,
            year,
            param,
            r#type: r#type.to_uppercase(),
            title,
            is_explicit,
            display_image,
            artists,
            items,
          })
        }
      }
      Err(_) => None,
    }
  }

  /**
   * [
   *    {
   *        "id": "1421904",
   *        "title": "Shut Up And Dance (Acoustic)",
   *        "type": "album",
   *        "perma_url": "https://www.jiosaavn.com/album/shut-up-and-dance-acoustic/Z0SzHx5Oxrk_",
   *        "image": "http://c.saavncdn.com/999/Shut-Up-And-Dance-Acoustic-English-2015-150x150.jpg",
   *        "explicit_content": "0",
   *        "list_count": "0",
   *        "more_info": {
   *            "artistMap": {
   *                "artists": [
   *                    {
   *                        "id": "577926",
   *                        "name": "WALK THE MOON",
   *                        "role": "",
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
  pub fn parse_related_albums(text: String) -> Option<Vec<JioSaavnAlbumPreview>> {
    match serde_json::from_str::<Value>(&text) {
      Ok(value) => {
        let mut album_pre_array: Vec<JioSaavnAlbumPreview> = Vec::new();
        let blank_array: &Vec<Value> = &Vec::new();

        for each_album_preview in value.as_array().unwrap_or(blank_array).into_iter() {
          if let Some(parsed_album_pre) =
            JioSaavnPartialParser::parse_album_preview(each_album_preview)
          {
            album_pre_array.push(parsed_album_pre);
          }
        }
        Some(album_pre_array)
      }
      Err(_) => None,
    }
  }
}

impl JioSaavnPartialParser {
  /**
   * {
   *    "album_id": "1070004",
   *    "album": "TALKING IS HARD",
   *    "album_url": "https://www.jiosaavn.com/album/student-of-the-year/QpcbT2oOB74_"
   * }
   */
  pub fn parse_album_basic_info(basic_album: &Value) -> Option<JioSaavnAlbumBasicInfo> {
    let url = basic_album["album_url"].get_string();
    let id = extract_id_from_url(url);
    let title = basic_album["album"].get_string();
    let param = basic_album["album_id"].get_string();

    if some_empty_string(&[id.clone(), title.clone(), param.clone()]) {
      None
    } else {
      Some(JioSaavnAlbumBasicInfo { id, title, param })
    }
  }

  /**
   * {
   *     "id": "1421904",
   *     "title": "Shut Up And Dance (Acoustic)",
   *     "type": "album",
   *     "perma_url": "https://www.jiosaavn.com/album/shut-up-and-dance-acoustic/Z0SzHx5Oxrk_",
   *     "image": "http://c.saavncdn.com/999/Shut-Up-And-Dance-Acoustic-English-2015-150x150.jpg",
   *     "explicit_content": "0",
   *     "list_count": "0",
   *     "more_info": {
   *         "artistMap": {
   *             "artists": [
   *                 {
   *                     "id": "577926",
   *                     "name": "WALK THE MOON",
   *                     "role": "",
   *                     "image": "http://c.saavncdn.com/artists/WALK_THE_MOON_150x150.jpg",
   *                     "type": "artist",
   *                     "perma_url": "https://www.jiosaavn.com/artist/walk-the-moon-songs/e0q9qdHeIys_"
   *                 }
   *             ]
   *         }
   *     }
   * }
   */
  pub fn parse_album_preview(album_preview: &Value) -> Option<JioSaavnAlbumPreview> {
    let url = album_preview["perma_url"].get_string();
    let id = extract_id_from_url(url);
    let title = album_preview["title"].get_string();
    let display_image = album_preview["image"].get_string();
    let param = album_preview["id"].get_string();
    let is_explicit = properize_explicit(album_preview["explicit_content"].get_string());

    let artist_array_default = &Vec::new();
    let artists_arr = album_preview["more_info"]["artistMap"]["artists"]
      .as_array()
      .unwrap_or(artist_array_default);
    let mut artists = Vec::new();

    for each_artist in artists_arr.into_iter() {
      if let Some(parsed_artist) = JioSaavnPartialParser::parse_artist_basic_info(each_artist) {
        artists.push(parsed_artist);
      }
    }

    if some_empty_string(&[id.clone(), title.clone(), param.clone()]) {
      None
    } else {
      Some(JioSaavnAlbumPreview {
        id,
        param,
        title,
        artists,
        is_explicit,
        display_image,
        r#type: "ALBUM_PREVIEW".to_string(),
      })
    }
  }
}
