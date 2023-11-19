/******* Search *******/

pub enum SearchFilter {
  SongResults,
  AlbumResults,
  ArtistResults,
  PlaylistResults,
}

impl SearchFilter {
  pub fn to_string(&self) -> String {
    match self {
      SearchFilter::SongResults => String::from("search.getResults"),
      SearchFilter::AlbumResults => String::from("search.getAlbumResults"),
      SearchFilter::ArtistResults => String::from("search.getArtistResults"),
      SearchFilter::PlaylistResults => String::from("search.getPlaylistResults"),
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      SearchFilter::SongResults => "search.getResults",
      SearchFilter::AlbumResults => "search.getAlbumResults",
      SearchFilter::ArtistResults => "search.getArtistResults",
      SearchFilter::PlaylistResults => "search.getPlaylistResults",
    }
  }
}

/******* WebApi *******/

pub enum WebApi {
  Get,
  GetLaunchData,
}

impl WebApi {
  pub fn to_string(&self) -> String {
    match self {
      WebApi::Get => String::from("webapi.get"),
      WebApi::GetLaunchData => String::from("webapi.getLaunchData"),
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      WebApi::Get => "webapi.get",
      WebApi::GetLaunchData => "webapi.getLaunchData",
    }
  }
}

/******* Autocomplete *******/

pub enum Autocomplete {
  Get
}

impl Autocomplete {
  pub fn to_string(&self) -> String {
    match self {
      Autocomplete::Get => String::from("autocomplete.get"),
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      Autocomplete::Get => "autocomplete.get",
    }
  }
}

/******* Reco *******/

pub enum Reco {
  GetAlbumReco,
  GetPlaylistReco
}

impl Reco {
  pub fn to_string(&self) -> String {
    match self {
      Reco::GetAlbumReco => String::from("reco.getAlbumReco"),
      Reco::GetPlaylistReco => String::from("reco.getPlaylistReco"),
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      Reco::GetAlbumReco => "reco.getAlbumReco",
      Reco::GetPlaylistReco => "reco.getPlaylistReco",
    }
  }
}

/******* Song *******/

pub enum Song {
  GenerateAuthToken
}

impl Song {
  pub fn to_string(&self) -> String {
    match self {
      Song::GenerateAuthToken => String::from("song.generateAuthToken"),
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      Song::GenerateAuthToken => "song.generateAuthToken",
    }
  }
}
