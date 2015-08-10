use std::fmt;

/// `HttpErrno` defines the encountered error during parsing.
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum HttpErrno {
    // Callback-related errors
    /// Error happened in message begin callback
    CBMessageBegin,
    /// Error happened in url callback
    CBUrl,
    /// Error happened in header field callback
    CBHeaderField,
    /// Error happened in header value callback
    CBHeaderValue,
    /// Error happened in headers complete callback
    CBHeadersComplete,
    /// Error happened in body callback
    CBBody,
    /// Error happened in message complete callback
    CBMessageComplete,
    /// Error happened in status callback
    CBStatus,

    // Parsing-related errors
    /// Invalid EOF state
    InvalidEofState,
    /// Header size is overflowed
    HeaderOverflow,
    /// Connection is closed
    ClosedConnection,
    /// Invalid HTTP version
    InvalidVersion,
    /// Invalid HTTP status
    InvalidStatus,
    /// Invalid HTTP method
    InvalidMethod,
    /// Invalid URL
    InvalidUrl,
    /// Invalid host
    InvalidHost,
    /// Invalid port
    InvalidPort,
    /// Invalid path
    InvalidPath,
    /// Invalid query string
    InvalidQueryString,
    /// Invalid fragment
    InvalidFragment,
    /// Line feed is expected
    LFExpected,
    /// Invalid header token
    InvalidHeaderToken,
    /// Invalid content length
    InvalidContentLength,
    /// Invalid chunk size
    InvalidChunkSize,
    /// Invalid constant
    InvalidConstant,
    /// Invalid internal state
    InvalidInternalState,
    /// Error happened in strict mode
    Strict,
    /// Error happened when the parser is paused
    Paused,
    /// Unkown error
    Unknown,
}

impl fmt::Display for HttpErrno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpErrno::CBMessageBegin => write!(f, "the on_message_begin callback failed"),
            HttpErrno::CBUrl => write!(f, "the on_url callback failed"),
            HttpErrno::CBHeaderField => write!(f, "the on_header_field callback failed"),
            HttpErrno::CBHeaderValue => write!(f, "the on_header_value callback failed"),
            HttpErrno::CBHeadersComplete => write!(f, "the on_headers_complete callback failed"),
            HttpErrno::CBBody => write!(f, "the on_body callback failed"),
            HttpErrno::CBMessageComplete => write!(f, "the on_message_complete callback failed"),
            HttpErrno::CBStatus => write!(f, "the on_status callback failed"),

            HttpErrno::InvalidEofState => write!(f, "stream ended at an unexpected time"),
            HttpErrno::HeaderOverflow => write!(f, "too many header bytes seen; overflow detected"),
            HttpErrno::ClosedConnection => write!(f, "data received after completed connection: close message"),
            HttpErrno::InvalidVersion => write!(f, "invalid HTTP version"),
            HttpErrno::InvalidStatus => write!(f, "invalid HTTP status code"),
            HttpErrno::InvalidMethod => write!(f, "invalid HTTP method"),
            HttpErrno::InvalidUrl => write!(f, "invalid URL"),
            HttpErrno::InvalidHost => write!(f, "invalid host"),
            HttpErrno::InvalidPort => write!(f, "invalid port"),
            HttpErrno::InvalidPath => write!(f, "invalid path"),
            HttpErrno::InvalidQueryString => write!(f, "invalid query string"),
            HttpErrno::InvalidFragment => write!(f, "invalid fragment"),
            HttpErrno::LFExpected => write!(f, "LF character expected"),
            HttpErrno::InvalidHeaderToken => write!(f, "invalid charater in header"),
            HttpErrno::InvalidContentLength => write!(f, "invalid character in content-length header"),
            HttpErrno::InvalidChunkSize => write!(f, "invalid character in chunk size header"),
            HttpErrno::InvalidConstant => write!(f, "invalid constant string"),
            HttpErrno::InvalidInternalState => write!(f, "encountered unexpected internal state"),
            HttpErrno::Strict => write!(f, "strict mode assertion failed"),
            HttpErrno::Paused => write!(f, "parser is parsed"),
            HttpErrno::Unknown => write!(f, "an unknown error occurred"),
        }
    }
}
