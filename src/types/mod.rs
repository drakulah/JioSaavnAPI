use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JioSaavnUnknownItemType {
  JioSaavnSong(JioSaavnSong),
  JioSaavnAlbumPreview(JioSaavnAlbumPreview),
  JioSaavnArtistPreview(JioSaavnArtistPreview),
  JioSaavnPlaylistPreview(JioSaavnPlaylistPreview),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnAlbumBasicInfo {
  pub id: String,
  pub param: String,
  pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnAlbumPreview {
  pub id: String,
  pub param: String,
  pub r#type: String,
  pub title: String,
  pub is_explicit: bool,
  pub display_image: String,
  pub artists: Vec<JioSaavnArtistPreview>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnAlbum {
  pub id: String,
  pub param: String,
  pub r#type: String,
  pub title: String,
  pub lang: String,
  pub year: String,
  pub is_explicit: bool,
  pub display_image: String,
  pub artists: Vec<JioSaavnArtistPreview>,
  pub items: Vec<JioSaavnSong>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
  pub artists: Vec<JioSaavnArtistPreview>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnSearch {
  pub total: i64,
  pub start: i64,
  pub results: Vec<JioSaavnUnknownItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnPlaylist {
  pub id: String,
  pub param: String,
  pub r#type: String,
  pub title: String,
  pub display_image: String,
  pub is_explicit: bool,
  pub items_count: i64,
  pub items: Vec<JioSaavnSong>,
  pub follower_count: i64,
  pub fan_count: i64,
  pub subtitles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnPlaylistPreview {
  pub id: String,
  pub param: String,
  pub title: String,
  pub subtitle: String,
  pub r#type: String,
  pub display_image: String,
  pub is_explicit: bool,
  pub items_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnItemContainer {
  pub title: String,
  pub subtitle: String,
  pub items: Vec<JioSaavnUnknownItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnHome {
  pub containers: Vec<JioSaavnItemContainer>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnAuthUrl {
  pub stream_url: String,
  pub audio_format: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnArtistPreview {
  pub id: String,
  pub title: String,
  pub param: String,
  pub display_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnArtistBio {
  pub title: String,
  pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JioSaavnArtist {
  pub title: String,
  pub param: String,
  pub r#type: String,
  pub display_image: String,
  pub follower_count: i64,
  pub fan_count: i64,
  pub is_verified: bool,
  pub date_of_birth: String,
  pub bio: Vec<JioSaavnArtistBio>,
  pub top_songs: Vec<JioSaavnSong>,
  pub top_albums: Vec<JioSaavnAlbumPreview>,
  pub dedicated_artist_playlists: Vec<JioSaavnPlaylistPreview>,
  pub featured_artist_playlists: Vec<JioSaavnPlaylistPreview>,
  pub singles: Vec<JioSaavnAlbumPreview>,
  pub latest_release: Vec<JioSaavnAlbumPreview>,
  pub similar_artists: Vec<JioSaavnArtistPreview>,
}
