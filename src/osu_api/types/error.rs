use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OsuError {
    pub error: String,
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Json(serde_json::error::Error),
    Osu(OsuError),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error::Json(err)
    }
}

impl From<OsuError> for Error {
    fn from(err: OsuError) -> Self {
        Error::Osu(err)
    }
}
