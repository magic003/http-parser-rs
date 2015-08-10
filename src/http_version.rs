use std::fmt;

/// HTTP protocol version.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct HttpVersion {
    /// Major version
    pub major: u8,
    /// Minor version
    pub minor: u8,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}
