# JioSaavnAPI

An unofficial JioSaavn API written in Rust.

## Features

Here are some things that you can fetch using this library:

- Song
- Home
- Album
- Artist
- Search
- Playlist
- Generate stream url

## Installation

Run the following Cargo command in your project directory:

```sh
cargo add jiosaavn_api
```

Or add the following line to your Cargo.toml:

```toml
jiosaavn_api = "0.1"
```

## Example

Here's an example of searching songs:

```rs
use jiosaavn_api::{
  api::{JioSaavn, JioSaavnConfig},
  enums::{call::SearchFilter, language::JioSaavnLanguage},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let config = JioSaavnConfig {
    music_lang: vec![
      JioSaavnLanguage::English,
      JioSaavnLanguage::Hindi
    ],
  };

  let client = JioSaavn::new(config);

  if let Ok(songs) = client
    .search(
      "walk the moon",
      1,
      20,
      SearchFilter::SongResults
    )
    .await
  {
    println!("{:?}", songs);
  }

  Ok(())
}
```

## License

MIT
