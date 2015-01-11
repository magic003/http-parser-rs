use std::string::ToString;

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

impl ToString for HttpErrno {
    fn to_string(&self) -> String {
        let desc = match *self {
            HttpErrno::CBMessageBegin => "the on_message_begin callback failed",
            HttpErrno::CBUrl => "the on_url callback failed",
            HttpErrno::CBHeaderField => "the on_header_field callback failed",
            HttpErrno::CBHeaderValue => "the on_header_value callback failed",
            HttpErrno::CBHeadersComplete => "the on_headers_complete callback failed",
            HttpErrno::CBBody => "the on_body callback failed",
            HttpErrno::CBMessageComplete => "the on_message_complete callback failed",
            HttpErrno::CBStatus => "the on_status callback failed",

            HttpErrno::InvalidEofState => "stream ended at an unexpected time",
            HttpErrno::HeaderOverflow => "too many header bytes seen; overflow detected",
            HttpErrno::ClosedConnection => "data received after completed connection: close message",
            HttpErrno::InvalidVersion => "invalid HTTP version",
            HttpErrno::InvalidStatus => "invalid HTTP status code",
            HttpErrno::InvalidMethod => "invalid HTTP method",
            HttpErrno::InvalidUrl => "invalid URL",
            HttpErrno::InvalidHost => "invalid host",
            HttpErrno::InvalidPort => "invalid port",
            HttpErrno::InvalidPath => "invalid path",
            HttpErrno::InvalidQueryString => "invalid query string",
            HttpErrno::InvalidFragment => "invalid fragment",
            HttpErrno::LFExpected => "LF character expected",
            HttpErrno::InvalidHeaderToken => "invalid charater in header",
            HttpErrno::InvalidContentLength => "invalid character in content-length header",
            HttpErrno::InvalidChunkSize => "invalid character in chunk size header",
            HttpErrno::InvalidConstant => "invalid constant string",
            HttpErrno::InvalidInternalState => "encountered unexpected internal state",
            HttpErrno::Strict => "strict mode assertion failed",
            HttpErrno::Paused => "parser is parsed",
            HttpErrno::Unknown => "an unknown error occurred",
        };
        desc.to_string()
    }
}
