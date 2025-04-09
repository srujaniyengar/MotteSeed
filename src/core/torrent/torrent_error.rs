use bencode::{DecoderError, streaming::Error};
use thiserror::Error;

//custom error enum for reading torrent operations
#[derive(Error, Debug)]
pub enum ReadTorrentError {
    //variant for decode errors with a display message
    #[error("Decoder error: {0}")]
    DecoderError(#[from] BDecoderError),

    //variant for streaming errors with a display message
    #[error("Streaming error: {0}")]
    StreamingError(#[from] BStreamingError),

    //logical error with a display message
    #[error("Logical error: {0}")]
    LogicError(String),

    //io error with a display message
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

//wrapper struct for DecoderError
#[derive(Debug)]
pub struct BDecoderError(DecoderError);

impl std::fmt::Display for BDecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::error::Error for BDecoderError {}

impl From<DecoderError> for BDecoderError {
    fn from(err: DecoderError) -> Self {
        BDecoderError(err)
    }
}

//wrapper struct for streaming::Error
#[derive(Debug)]
pub struct BStreamingError(Error);

impl std::fmt::Display for BStreamingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::error::Error for BStreamingError {}

impl From<Error> for BStreamingError {
    fn from(err: Error) -> Self {
        BStreamingError(err)
    }
}
