use crate::http::uri;
use hyper::{http::request::Builder, Request};

pub struct DefaultClient {}

impl DefaultClient {
  pub fn uri_builder() -> uri::Uri {
    uri::Uri::builder()
      .scheme("https")
      .host("www.jiosaavn.com")
      .path("/api.php")
      .search_param("ctx", "wap6dot0")
      .search_param("api_version", "4")
      .search_param("_format", "json")
      .search_param("_marker", "0")
  }

  pub fn req_builder() -> Builder {
    Request::builder().header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0")
  }
}
