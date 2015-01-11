extern crate http_parser;


use self::http_parser::{HttpParser, HttpParserType};

mod helper;

#[test]
fn test_request_line() {
    let line : &str = "GET / HTTP/1.1\r\n";
    test_first_line(HttpParserType::Request, line.as_bytes());
}

#[test]
fn test_status_line() {
    let line : &str = "HTTP/1.0 200 OK\r\n";
    test_first_line(HttpParserType::Response, line.as_bytes());
}

fn test_first_line(tp : HttpParserType, data : &[u8]) {
    let mut hp : HttpParser = HttpParser::new(tp);
    let mut cb = helper::CallbackEmpty;
    let parsed : u64 = hp.execute(&mut cb, data);
    assert_eq!(parsed, data.len() as u64);
}
