use hyper::Body;

use crate::{
  client::default_client::DefaultClient,
  enums::call,
  http::{errors::ErrorKind, http::fetch},
  parser::JioSaavnResponseParser,
  types::{JioSaavnAlbum, JioSaavnAlbumPreview},
};

use super::JioSaavn;

impl JioSaavn {
  pub async fn album(&self, token: &str) -> Result<JioSaavnAlbum, ErrorKind> {
    let uri_builder = DefaultClient::uri_builder()
      .search_param("token", token)
      .search_param("type", "album")
      .search_param("__call", call::WebApi::Get.as_str());

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
          Ok(fetched) => {
            if let Some(res) = JioSaavnResponseParser::parse_album(fetched.to_string()) {
              Ok(res)
            } else {
              Err(ErrorKind::NoData)
            }
          }
          Err(e) => Err(e),
        }
      }
      Err(_) => Err(ErrorKind::InvalidUri),
    }
  }

  pub async fn related_album(&self, param: &str) -> Result<Vec<JioSaavnAlbumPreview>, ErrorKind> {
    let uri_builder = DefaultClient::uri_builder()
      .search_param("albumid", param)
      .search_param("__call", call::Reco::GetAlbumReco.as_str());

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
          Ok(fetched) => Ok(JioSaavnResponseParser::parse_related_albums(
            fetched.to_string(),
          )),
          Err(e) => Err(e),
        }
      }
      Err(_) => Err(ErrorKind::InvalidUri),
    }
  }
}
