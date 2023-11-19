pub fn some_empty_string<S: AsRef<str>>(texts: &[S]) -> bool {
  for e in texts.into_iter() {
    if e.as_ref().is_empty() {
      return true;
    }
  }
  false
}
