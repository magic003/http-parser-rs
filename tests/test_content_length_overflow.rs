extern crate http_parser;

use http_parser::{HttpParser, HttpParserType, HttpErrno};

pub mod helper;

macro_rules! content_length(
    ($len:expr) => (
        format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", $len);
    );
);

macro_rules! chunk_content(
    ($len:expr) => (
        format!("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n{}\r\n...", $len);
    );
);

#[test]
fn test_header_content_length_overflow() {
    let a = content_length!("1844674407370955160");     // 2^64 / 10 - 1
    let b = content_length!("18446744073709551615");    // 2^64 - 1
    let c = content_length!("18446744073709551616");    // 2^64

    test_content_length_overflow(a.as_bytes(), true);
    test_content_length_overflow(b.as_bytes(), false);
    test_content_length_overflow(c.as_bytes(), false);
}

#[test]
fn test_chunk_content_length_overflow() {
    let a = chunk_content!("FFFFFFFFFFFFFFE");          // 2^64 / 16 - 1
    let b = chunk_content!("FFFFFFFFFFFFFFFF");         // 2^64 - 1
    let c = chunk_content!("10000000000000000");        // 2^64

    test_content_length_overflow(a.as_bytes(), true);
    test_content_length_overflow(b.as_bytes(), false);
    test_content_length_overflow(c.as_bytes(), false);
}

fn test_content_length_overflow(data: &[u8], expect_ok: bool) {
    let mut hp = HttpParser::new(HttpParserType::Response);
    let mut cb = helper::CallbackEmpty;

    hp.execute(&mut cb, data);

    if expect_ok {
        assert!(hp.errno.is_none());
    } else {
        assert!(hp.errno == Option::Some(HttpErrno::InvalidContentLength));
    }
}
