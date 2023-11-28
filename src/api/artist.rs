use hyper::Body;

use crate::{
  client::default_client::DefaultClient,
  enums::{artist_category::ArtistCategory, call, sort_order::SortOrder},
  http::{errors::ErrorKind, http::fetch},
};

use super::JioSaavn;

impl JioSaavn {
  pub async fn artist(
    &self,
    token: &str,
    page: i16,
    song_count: i16,
    album_count: i16,
    sort_order: SortOrder,
    category: ArtistCategory,
  ) -> Result<String, ErrorKind> {
    let uri_builder = DefaultClient::uri_builder()
      .search_param("token", token)
      .search_param("type", "artist")
      .search_param("p", page.to_string().as_str())
      .search_param("n_song", song_count.to_string().as_str())
      .search_param("n_album", album_count.to_string().as_str())
      .search_param("__call", call::WebApi::Get.as_str())
      .search_param("sort_order", sort_order.as_str())
      .search_param("category", category.as_str())
      .search_param("subtype", "");

    match uri_builder.build() {
      Ok(uri) => {
        match fetch(
          DefaultClient::req_builder()
            .method("GET")
            .uri(uri)
            .header("Cookie", self.config_as_cookie())
            .body(Body::empty()),
        )
        .await
        {
          Ok(fetched) => Ok(fetched.to_string()),
          Err(e) => Err(e),
        }
      }
      Err(_) => Err(ErrorKind::InvalidUri),
    }
  }
}
