extern crate core;

use self::core::fmt;

#[derive(PartialEq, Eq, Copy)]
pub struct HttpVersion {
    pub major: u8,
    pub minor: u8,
}

impl fmt::Show for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format!("{}.{}", self.major, self.minor).fmt(f)
    }
}
