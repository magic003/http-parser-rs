extern crate http_parser;

use http_parser::{HttpParserType, HttpParser, HttpParserCallback};

mod helper;

#[test]
fn test_no_overflow_long_body_request() {
    test_no_overflow_long_body(HttpParserType::HttpRequest, 1000);
    test_no_overflow_long_body(HttpParserType::HttpRequest, 100000);
}

#[test]
fn test_no_overflow_long_body_response() {
    test_no_overflow_long_body(HttpParserType::HttpResponse, 1000);
    test_no_overflow_long_body(HttpParserType::HttpResponse, 100000);
}

fn test_no_overflow_long_body(tp: HttpParserType, length: u64) {
    let mut hp = HttpParser::new(tp);
    let mut cb = helper::CallbackEmpty;
    
    let line = if tp == HttpParserType::HttpRequest {
        "POST / HTTP/1.0"
    } else {
        "HTTP/1.0 200 OK"
    };

    let headers = format!("{}\r\nConnection: Keep-Alive\r\nContent-Length: {}\r\n\r\n",
                          line, length);

    let mut parsed = hp.execute(&mut cb, headers.as_bytes());
    assert_eq!(parsed, headers.len() as u64); 

    for i in range(0, length) {
        parsed = hp.execute(&mut cb, [b'a'].as_slice());
        assert_eq!(parsed, 1 as u64);
    }

    parsed = hp.execute(&mut cb, headers.as_bytes());
    assert_eq!(parsed, headers.len() as u64);
}
