use hyper::Body;

use crate::{
  client::default_client::DefaultClient,
  enums::call,
  http::{errors::ErrorKind, http::fetch},
  parser::JioSaavnResponseParser,
  types::JioSaavnSearch,
};

use super::JioSaavn;

impl JioSaavn {
  pub async fn search(
    &self,
    query: &str,
    page: i16,
    results_count: i16,
    filter_param: call::SearchFilter,
  ) -> Result<JioSaavnSearch, ErrorKind> {
    let uri_builder = DefaultClient::uri_builder()
      .search_param("q", query)
      .search_param("p", page.to_string().as_str())
      .search_param("n", results_count.to_string().as_str())
      .search_param("__call", filter_param.as_str());

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
            if let Some(res) = JioSaavnResponseParser::parse_search_results(fetched.to_string()) {
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
