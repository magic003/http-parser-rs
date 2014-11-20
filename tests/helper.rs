extern crate http_parser;

use self::http_parser::HttpParserCallback;

pub struct CallbackEmpty;

impl HttpParserCallback for CallbackEmpty {}
