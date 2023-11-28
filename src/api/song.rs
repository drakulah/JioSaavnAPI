use hyper::Body;

use crate::{
  client::default_client::DefaultClient,
  enums::call,
  http::{errors::ErrorKind, http::fetch},
};

use super::JioSaavn;

impl JioSaavn {
  pub async fn song(&self, token: &str) -> Result<String, ErrorKind> {
    let uri_builder = DefaultClient::uri_builder()
      .search_param("token", token)
      .search_param("type", "song")
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
          Ok(fetched) => Ok(fetched.to_string()),
          Err(e) => Err(e),
        }
      }
      Err(_) => Err(ErrorKind::InvalidUri),
    }
  }
}
