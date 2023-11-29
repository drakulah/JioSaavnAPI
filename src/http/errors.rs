pub enum ErrorKind {
  InvalidRequest,
  InvalidUri,

  ResponseBodyError,

  BytesConversion,
  StringConversion,
  JsonConversion,

  NoData,
}

impl ErrorKind {
  pub fn as_str(&self) -> &str {
    match self {
      ErrorKind::InvalidRequest => "InvalidRequest",
      ErrorKind::InvalidUri => "InvalidUri",
      ErrorKind::ResponseBodyError => "ResponseBodyError",
      ErrorKind::BytesConversion => "BytesConversion",
      ErrorKind::StringConversion => "StringConversion",
      ErrorKind::JsonConversion => "JsonConversion",
      ErrorKind::NoData => "NoData",
    }
  }
}
