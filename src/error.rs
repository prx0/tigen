use std::fmt::Display;

#[derive(Debug)]
pub struct DecodingError(pub String);

impl Display for DecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl std::error::Error for DecodingError {}

#[derive(Debug)]
pub enum Error {
    Decoding(DecodingError),
    Templating(tera::Error),
    IO(std::io::Error),
    Nom(String),
}

impl From<DecodingError> for Error {
    fn from(err: DecodingError) -> Self {
        Self::Decoding(err)
    }
}

impl From<tera::Error> for Error {
    fn from(err: tera::Error) -> Self {
        Self::Templating(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl std::error::Error for Error {}
