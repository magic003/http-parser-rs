extern crate core;

use self::core::fmt;

#[derive(PartialEq, Eq, Copy)]
pub enum HttpErrno {
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

impl fmt::String for HttpErrno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpErrno::CBMessageBegin => "the on_message_begin callback failed".fmt(f),
            HttpErrno::CBUrl => "the on_url callback failed".fmt(f),
            HttpErrno::CBHeaderField => "the on_header_field callback failed".fmt(f),
            HttpErrno::CBHeaderValue => "the on_header_value callback failed".fmt(f),
            HttpErrno::CBHeadersComplete => "the on_headers_complete callback failed".fmt(f),
            HttpErrno::CBBody => "the on_body callback failed".fmt(f),
            HttpErrno::CBMessageComplete => "the on_message_complete callback failed".fmt(f),
            HttpErrno::CBStatus => "the on_status callback failed".fmt(f),

            HttpErrno::InvalidEofState => "stream ended at an unexpected time".fmt(f),
            HttpErrno::HeaderOverflow => "too many header bytes seen; overflow detected".fmt(f),
            HttpErrno::ClosedConnection => "data received after completed connection: close message".fmt(f),
            HttpErrno::InvalidVersion => "invalid HTTP version".fmt(f),
            HttpErrno::InvalidStatus => "invalid HTTP status code".fmt(f),
            HttpErrno::InvalidMethod => "invalid HTTP method".fmt(f),
            HttpErrno::InvalidUrl => "invalid URL".fmt(f),
            HttpErrno::InvalidHost => "invalid host".fmt(f),
            HttpErrno::InvalidPort => "invalid port".fmt(f),
            HttpErrno::InvalidPath => "invalid path".fmt(f),
            HttpErrno::InvalidQueryString => "invalid query string".fmt(f),
            HttpErrno::InvalidFragment => "invalid fragment".fmt(f),
            HttpErrno::LFExpected => "LF character expected".fmt(f),
            HttpErrno::InvalidHeaderToken => "invalid charater in header".fmt(f),
            HttpErrno::InvalidContentLength => "invalid character in content-length header".fmt(f),
            HttpErrno::InvalidChunkSize => "invalid character in chunk size header".fmt(f),
            HttpErrno::InvalidConstant => "invalid constant string".fmt(f),
            HttpErrno::InvalidInternalState => "encountered unexpected internal state".fmt(f),
            HttpErrno::Strict => "strict mode assertion failed".fmt(f),
            HttpErrno::Paused => "parser is parsed".fmt(f),
            HttpErrno::Unknown => "an unknown error occurred".fmt(f),
        }
    }
}
