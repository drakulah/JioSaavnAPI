use hyper::Body;

use crate::{
  client::default_client::DefaultClient,
  enums::call,
  http::{errors::ErrorKind, http::fetch},
  parser::JioSaavnResponseParser,
  types::JioSaavnHome,
};

use super::JioSaavn;

impl JioSaavn {
  pub async fn home(&self) -> Result<JioSaavnHome, ErrorKind> {
    let uri_builder =
      DefaultClient::uri_builder().search_param("__call", call::WebApi::GetLaunchData.as_str());

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
          Ok(fetched) => Ok(JioSaavnResponseParser::parse_home(fetched.to_string())),
          Err(e) => Err(e),
        }
      }
      Err(_) => Err(ErrorKind::InvalidUri),
    }
  }
}
