use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct HttpVersion {
    pub major: u8,
    pub minor: u8,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}
