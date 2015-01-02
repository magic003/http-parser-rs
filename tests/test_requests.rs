extern crate http_parser;

use std::default::Default;

use http_parser::{HttpParser, HttpParserType, HttpErrno};

mod helper;

#[test]
fn test_requests() {
    test_simple("GET / HTP/1.1\r\n\r\n", HttpErrno::InvalidVersion);

    // Well-formed but incomplete
    test_simple("GET / HTTP/1.1\r\n\
                 Content-Type: text/plain\r\n\
                 Content-Length: 6\r\n\
                 \r\n\
                 fooba", HttpErrno::Ok);
}

fn test_simple(buf: &str, err_expected: HttpErrno) {
    let mut hp = HttpParser::new(HttpParserType::HttpRequest);

    let mut cb = helper::CallbackRegular{..Default::default()};
    cb.messages.push(helper::Message{..Default::default()});

    hp.execute(&mut cb, buf.as_bytes());
    let err = hp.errno;
    cb.currently_parsing_eof = true;
    hp.execute(&mut cb, &[]);

    assert!(err_expected == err || 
            (hp.strict && (err_expected == HttpErrno::Ok || err == HttpErrno::Strict)),
            "\n*** test_simple expected {}, but saw {} ***\n\n{}\n", 
            err_expected.to_string(), err.to_string(), buf);
}
