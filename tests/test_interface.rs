extern crate http_parser;

use self::http_parser::{HttpParser, HttpParserType, HttpParserCallback};

#[test]
fn test_interface() {
    let mut hp = HttpParser::new(HttpParserType::HttpBoth);
    
    struct Callback;

    impl HttpParserCallback for Callback {
        fn on_message_complete(&mut self, parser : &HttpParser) -> Result<i8, &str> {
            Ok(1)
        }
    }

    let mut cb = Callback;
    hp.execute(&mut cb, [b'a', b'b', b'c'].as_slice());
}
