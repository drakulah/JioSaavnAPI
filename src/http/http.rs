use hyper::{
  body::{to_bytes, Bytes},
  http::Error,
  Body, HeaderMap, Request,
};
use serde_json::Value;

use super::errors::ErrorKind;

#[derive(Debug, Clone)]
pub struct RequestInit {
  body_bytes: Bytes,
  headers: HeaderMap,
}

impl RequestInit {
  pub fn to_string(&self) -> String {
    let body_vec = self.body_bytes.to_vec();
    String::from_utf8_lossy(&body_vec).to_string()
  }

  pub fn to_json<T>(&self) -> Result<Value, ErrorKind> {
    let text = self.to_string();
    match serde_json::from_str::<Value>(text.as_str()) {
      Ok(json) => Ok(json),
      Err(_) => Err(ErrorKind::JsonConversion),
    }
  }

  pub fn headers(&self) -> HeaderMap {
    self.headers.clone()
  }
}

pub async fn fetch(req: Result<Request<Body>, Error>) -> Result<RequestInit, ErrorKind> {
  let req_client = super::client::default_client();

  match req {
    Ok(valid_req) => match req_client.request(valid_req).await {
      Ok(res) => {
        let headers = res.headers().clone();
        match to_bytes(res.into_body()).await {
          Ok(body_bytes) => Ok(RequestInit {
            body_bytes,
            headers,
          }),
          Err(_) => Err(ErrorKind::BytesConversion),
        }
      }
      Err(_) => Err(ErrorKind::ResponseBodyError),
    },
    Err(_) => Err(ErrorKind::InvalidRequest),
  }
}
