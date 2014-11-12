#[deriving(PartialEq, Eq)]
pub enum HttpErrno {
    // No error
    Ok,

    // Callback-related errors
    CBMessageBegin,
    CBUrl,
    CBHeaderField,
    CBHeaderValue,
    CBHeadersComplete,
    CBBody,
    CBMessageComplete,
    CBStatus,

    // Parsing-related errors
    InvalidEofState,
    HeaderOverflow,
    ClosedConnection,
    InvalidVersion,
    InvalidStatus,
    InvalidMethod,
    InvalidUrl,
    InvalidHost,
    InvalidPort,
    InvalidPath,
    InvalidQueryString,
    InvalidFragment,
    LFExpected,
    InvalidHeaderToken,
    InvalidContentLength,
    InvalidChunkSize,
    InvalidConstant,
    InvalidInternalState,
    Strict,
    Paused,
    Unknown,
}
