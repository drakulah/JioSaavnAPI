pub fn extract_id_from_url<S: AsRef<str>>(url: S) -> String {
  match url.as_ref().split("/").last() {
    Some(id) => id.to_owned(),
    None => String::new(),
  }
}

pub fn properize_explicit<S: AsRef<str>>(explicit: S) -> bool {
  match explicit.as_ref() {
    "1" => true,
    _ => false,
  }
}