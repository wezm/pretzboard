extern crate reqwest;
extern crate serde_json;

use std::{io, num, str};
use weather::WeatherError;

#[derive(Fail, Debug)]
pub enum PretzboardError {
    #[fail(display = "I/O error")] IoError(io::Error),
    #[fail(display = "HTTP error")] HttpError(reqwest::Error),
    #[fail(display = "UTF-8 parse error")] ParseError(str::Utf8Error),
    #[fail(display = "Flickr error")] FlickrError(FlickrError),
    #[fail(display = "JSON error")] JsonError(serde_json::Error),
    #[fail(display = "Graphics error")] GraphicsError,
    #[fail(display = "Weather error")] WeatherError(WeatherError),
}

impl From<str::Utf8Error> for PretzboardError {
    fn from(err: str::Utf8Error) -> Self {
        PretzboardError::ParseError(err)
    }
}

impl From<FlickrError> for PretzboardError {
    fn from(err: FlickrError) -> Self {
        PretzboardError::FlickrError(err)
    }
}

impl From<WeatherError> for PretzboardError {
    fn from(err: WeatherError) -> Self {
        PretzboardError::WeatherError(err)
    }
}

impl From<serde_json::Error> for PretzboardError {
    fn from(err: serde_json::Error) -> Self {
        PretzboardError::JsonError(err)
    }
}

impl From<io::Error> for PretzboardError {
    fn from(err: io::Error) -> Self {
        PretzboardError::IoError(err)
    }
}

impl From<reqwest::Error> for PretzboardError {
    fn from(err: reqwest::Error) -> Self {
        PretzboardError::HttpError(err)
    }
}

#[derive(Fail, Debug)]
pub enum FlickrError {
    #[fail(display = "The request was rejected")] AuthenticationError,
    #[fail(display = "JSON error")] JsonError(serde_json::Error),
    #[fail(display = "I/O error")] IoError(io::Error),
    #[fail(display = "HTTP error")] HttpError(reqwest::Error),
    #[fail(display = "URL error")] UrlError(reqwest::UrlError),
    #[fail(display = "Parse error")] ParseError(num::ParseIntError),
}

impl From<serde_json::Error> for FlickrError {
    fn from(err: serde_json::Error) -> Self {
        FlickrError::JsonError(err)
    }
}

impl From<io::Error> for FlickrError {
    fn from(err: io::Error) -> Self {
        FlickrError::IoError(err)
    }
}

impl From<reqwest::Error> for FlickrError {
    fn from(err: reqwest::Error) -> Self {
        FlickrError::HttpError(err)
    }
}

impl From<reqwest::UrlError> for FlickrError {
    fn from(err: reqwest::UrlError) -> Self {
        FlickrError::UrlError(err)
    }
}

impl From<num::ParseIntError> for FlickrError {
    fn from(err: num::ParseIntError) -> Self {
        FlickrError::ParseError(err)
    }
}
