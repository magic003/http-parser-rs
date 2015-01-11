pub enum Flags {
    CHUNKED                 = 1is << 0,
    CONNECTION_KEEP_ALIVE   = 1is << 1,
    CONNECTION_CLOSE        = 1is << 2,
    TRAILING                = 1is << 3,
    UPGRADE                 = 1is << 4,
    SKIPBODY                = 1is << 5,
}

impl Flags {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
