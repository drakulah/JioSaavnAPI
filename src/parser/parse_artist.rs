use serde_json::Value;

use crate::{
  types::{
    JioSaavnAlbumPreview, JioSaavnArtist, JioSaavnArtistBio, JioSaavnArtistPreview,
    JioSaavnPlaylistPreview, JioSaavnSong,
  },
  utils::{array::some_empty_string, parser::extract_id_from_url},
};

use super::{JioSaavnPartialParser, JioSaavnResponseParser, ValueExtras};

impl JioSaavnResponseParser {
  pub fn parse_artist(text: String) -> Option<JioSaavnArtist> {
    match serde_json::from_str::<Value>(&text) {
      Ok(artist) => {
        let param = artist["artistId"].get_string();
        let r#type = artist["type"].get_string().to_uppercase();
        let title = artist["name"].get_string();
        let display_image = artist["image"].get_string();
        let follower_count = artist["follower_count"].get_str_as_int();
        let fan_count = artist["fan_count"].get_str_as_int();
        let is_verified = artist["isVerified"].as_bool().unwrap_or(false);
        let date_of_birth = artist["dob"].get_string();
        let bio_text = artist["bio"].get_string();
        let bio = JioSaavnPartialParser::parse_artist_bio(bio_text);

        let top_songs_arr = artist["topSongs"].get_arr();
        let top_albums_arr = artist["topAlbums"].get_arr();
        let dedicated_artist_playlist_arr = artist["dedicated_artist_playlist"].get_arr();
        let featured_artist_playlist_arr = artist["featured_artist_playlist"].get_arr();
        let singles_arr = artist["singles"].get_arr();
        let latest_release_arr = artist["latest_release"].get_arr();
        let similar_artists_arr = artist["similarArtists"].get_arr();

        let mut top_songs: Vec<JioSaavnSong> = Vec::new();
        let mut top_albums: Vec<JioSaavnAlbumPreview> = Vec::new();
        let mut dedicated_artist_playlists: Vec<JioSaavnPlaylistPreview> = Vec::new();
        let mut featured_artist_playlists: Vec<JioSaavnPlaylistPreview> = Vec::new();
        let mut singles: Vec<JioSaavnAlbumPreview> = Vec::new();
        let mut latest_release: Vec<JioSaavnAlbumPreview> = Vec::new();
        let mut similar_artists: Vec<JioSaavnArtistPreview> = Vec::new();

        for maybe_song in top_songs_arr.into_iter() {
          if let Some(song) = JioSaavnPartialParser::parse_song(&maybe_song) {
            top_songs.push(song);
          }
        }

        for maybe_album_pre in top_albums_arr.into_iter() {
          if let Some(album_pre) = JioSaavnPartialParser::parse_album_preview(&maybe_album_pre) {
            top_albums.push(album_pre);
          }
        }

        for maybe_playlist in dedicated_artist_playlist_arr.into_iter() {
          if let Some(playlist_pre) = JioSaavnPartialParser::parse_playlist_preview(&maybe_playlist)
          {
            dedicated_artist_playlists.push(playlist_pre);
          }
        }

        for maybe_playlist in featured_artist_playlist_arr.into_iter() {
          if let Some(playlist_pre) = JioSaavnPartialParser::parse_playlist_preview(&maybe_playlist)
          {
            featured_artist_playlists.push(playlist_pre);
          }
        }

        for maybe_album_pre in singles_arr.into_iter() {
          if let Some(album_pre) = JioSaavnPartialParser::parse_album_preview(&maybe_album_pre) {
            singles.push(album_pre);
          }
        }

        for maybe_album_pre in latest_release_arr.into_iter() {
          if let Some(album_pre) = JioSaavnPartialParser::parse_album_preview(&maybe_album_pre) {
            latest_release.push(album_pre);
          }
        }

        for maybe_artist_pre in similar_artists_arr.into_iter() {
          if let Some(artist_pre) = JioSaavnPartialParser::parse_artist_preview(&maybe_artist_pre) {
            similar_artists.push(artist_pre);
          }
        }

        if some_empty_string(&[title.clone(), param.clone()]) || r#type != "ARTIST" {
          return None;
        }

        Some(JioSaavnArtist {
          title,
          param,
          r#type,
          display_image,
          follower_count,
          fan_count,
          is_verified,
          date_of_birth,
          bio,
          top_songs,
          top_albums,
          dedicated_artist_playlists,
          featured_artist_playlists,
          singles,
          latest_release,
          similar_artists,
        })
      }
      Err(_) => None,
    }
  }
}

impl JioSaavnPartialParser {
  pub fn parse_artist_preview(artist: &Value) -> Option<JioSaavnArtistPreview> {
    let param = artist["id"].get_string();
    let perma_url = artist["perma_url"].get_string();
    let title = artist["name"].get_string();
    let display_image = artist["image"].get_string();
    let id = extract_id_from_url(perma_url);
    let r#type = artist["type"].get_string().to_uppercase();

    if some_empty_string(&[id.clone(), title.clone(), param.clone()]) || r#type != "ARTIST" {
      return None;
    }

    Some(JioSaavnArtistPreview {
      id,
      title,
      param,
      display_image,
    })
  }

  pub fn parse_artist_bio(bio: String) -> Vec<JioSaavnArtistBio> {
    let mut bios: Vec<JioSaavnArtistBio> = Vec::new();
    match serde_json::from_str::<Value>(&bio) {
      Ok(bio_arr) => {
        for maybe_bio in bio_arr.get_arr().into_iter() {
          let title = maybe_bio["title"].get_string();
          let text = maybe_bio["text"].get_string();

          if !some_empty_string(&[title.clone(), text.clone()]) {
            bios.push(JioSaavnArtistBio { title, text });
          }
        }

        bios
      }
      Err(_) => bios,
    }
  }
}
