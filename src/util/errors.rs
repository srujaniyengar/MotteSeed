use bencode::streaming::Error as BencStreamingError;

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
