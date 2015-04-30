pub enum Flags {
    CHUNKED                 = 1isize << 0,
    CONNECTION_KEEP_ALIVE   = 1isize << 1,
    CONNECTION_CLOSE        = 1isize << 2,
    TRAILING                = 1isize << 3,
    UPGRADE                 = 1isize << 4,
    SKIPBODY                = 1isize << 5,
}

impl Flags {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
