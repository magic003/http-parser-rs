use parser::HttpParser;

pub trait HttpParserCallback {
    fn on_message_begin(&mut self, parser : &mut HttpParser) -> Result<i8, &str> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn on_url(&mut self, parser : &mut HttpParser, data : &[u8],) -> Result<i8, &str> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn on_status(&mut self, parser : &mut HttpParser, data : &[u8]) -> Result<i8, &str> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn on_header_field(&mut self, parser : &mut HttpParser, data : &[u8]) -> Result<i8, &str> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn on_header_value(&mut self, parser : &mut HttpParser, data : &[u8]) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_headers_complete(&mut self, parser : &mut HttpParser) -> Result<i8, &str> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn on_body(&mut self, parser : &mut HttpParser, data : &[u8]) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_message_complete(&mut self, parser : &mut HttpParser) -> Result<i8, &str> {
        Ok(0)
    }
}
