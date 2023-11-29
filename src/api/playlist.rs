use hyper::Body;

use crate::{
  client::default_client::DefaultClient,
  enums::call,
  http::{errors::ErrorKind, http::fetch},
  parser::JioSaavnResponseParser,
  types::JioSaavnPlaylist,
};

use super::JioSaavn;

impl JioSaavn {
  pub async fn playlist(
    &self,
    token: &str,
    page: i16,
    song_count: i16,
  ) -> Result<JioSaavnPlaylist, ErrorKind> {
    let uri_builder = DefaultClient::uri_builder()
      .search_param("token", token)
      .search_param("type", "playlist")
      .search_param("p", page.to_string().as_str())
      .search_param("n", song_count.to_string().as_str())
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
            if let Some(res) = JioSaavnResponseParser::parse_playlist(fetched.to_string()) {
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
