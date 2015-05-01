pub enum Flags {
    Chunked                 = 1isize << 0,
    ConnectionKeepAlive     = 1isize << 1,
    ConnectionClose         = 1isize << 2,
    Trailing                = 1isize << 3,
    Upgrade                 = 1isize << 4,
    SkipBody                = 1isize << 5,
}

impl Flags {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
