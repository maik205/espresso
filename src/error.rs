
pub enum EspressoRequestError {
    MalformedRequest(String),
    IncompleteRequest(String),

}

pub enum EspressoProcessingError {
    HandleBeforeListen,
    FailedThreadPool,
    ConnectionClosed
}
