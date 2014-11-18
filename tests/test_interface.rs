extern crate http_parser;

use self::http_parser::{HttpParser, HttpParserType, HttpParserCallback};

#[test]
fn test_interface() {
    let mut hp = HttpParser::new(HttpParserType::HttpBoth);
    
    struct Callback;

    impl HttpParserCallback for Callback {
        fn on_message_complete(&self) -> Result<i8, &str> {
            Ok(1)
        }
    }

    let cb = Callback;
    hp.execute(cb, [b'a', b'b', b'c']);
}
