use hyper::Body;

use crate::{
  client::default_client::DefaultClient,
  enums::call,
  http::{errors::ErrorKind, http::fetch},
};

use super::JioSaavn;

impl JioSaavn {
  pub async fn related_playlist(&self, playlist_id: &str) -> Result<String, ErrorKind> {
    let uri_builder = DefaultClient::uri_builder()
      .search_param("listid", playlist_id)
      .search_param("__call", call::Reco::GetPlaylistReco.as_str());

    match uri_builder.build() {
      Ok(uri) => {
        match fetch(
          DefaultClient::req_builder()
            .method("GET")
            .uri(uri)
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
