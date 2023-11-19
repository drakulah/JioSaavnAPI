pub fn decode<S: AsRef<str>>(inp: S) -> String {
  let input = inp.as_ref();
  let mut result = String::with_capacity(input.len());
  let mut chars = input.chars().peekable();

  while let Some(c) = chars.next() {
    if c == '&' {
      let mut entity = String::new();

      while let Some(&next_char) = chars.peek() {
        if next_char == ';' {
          chars.next();
          break;
        }

        entity.push(chars.next().unwrap());
      }

      let decoded_char = match entity.as_str() {
        "amp" => '&',
        "lt" => '<',
        "gt" => '>',
        _ => {
          result.push('&');
          result.push_str(&entity);
          continue;
        }
      };
      result.push(decoded_char);
    } else {
      result.push(c);
    }
  }
  result
}
