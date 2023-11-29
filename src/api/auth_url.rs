use hyper::Body;

use crate::{
  client::default_client::DefaultClient,
  enums::{bitrate::JioSaavnBitrate, call},
  http::{errors::ErrorKind, http::fetch},
  parser::JioSaavnResponseParser,
  types::JioSaavnAuthUrl,
};

use super::JioSaavn;

impl JioSaavn {
  pub async fn auth_url(
    &self,
    enc_media_url: &str,
    bitrate: JioSaavnBitrate,
  ) -> Result<JioSaavnAuthUrl, ErrorKind> {
    let uri_builder = DefaultClient::uri_builder()
      .search_param("url", enc_media_url)
      .search_param("bitrate", bitrate.to_str())
      .search_param("__call", call::Song::GenerateAuthToken.as_str());

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
          Ok(fetched) => {
            if let Some(res) = JioSaavnResponseParser::parse_auth_url(fetched.to_string()) {
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
}
