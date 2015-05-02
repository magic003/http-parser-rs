extern crate http_parser;

use self::http_parser::*;

#[test]
fn test_interface() {
    let mut hp = HttpParser::new(HttpParserType::Both);
    
    struct Callback;

    impl HttpParserCallback for Callback {
        fn on_message_complete(&mut self, _ : &mut HttpParser) -> CallbackResult {
            Ok(CallbackDecision::Nothing)
        }
    }

    let mut cb = Callback;
    hp.execute(&mut cb, &[b'a', b'b', b'c']);
}
