pub mod flags {
    pub const CHUNKED : u8                 = 1u8 << 0;
    pub const CONNECTION_KEEP_ALIVE : u8    = 1u8 << 1;
    pub const CONNECTION_CLOSE : u8         = 1u8 << 2;
    pub const TRAILING : u8                 = 1u8 << 3;
    pub const UPGRADE : u8                  = 1u8 << 4;
    pub const SKIPBODY : u8                 = 1u8 << 5;
}
