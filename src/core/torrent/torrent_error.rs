use bencode::streaming::Error as BencStreamingError;
use thiserror::Error;

//custom error enum for reading torrent operations
#[derive(Error, Debug)]
pub enum ReadTorrentError {
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

//wrapper struct for streaming::Error
#[derive(Debug)]
pub struct BStreamingError(BencStreamingError);

impl std::fmt::Display for BStreamingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::error::Error for BStreamingError {}

impl From<BencStreamingError> for BStreamingError {
    fn from(err: BencStreamingError) -> Self {
        BStreamingError(err)
    }
}
