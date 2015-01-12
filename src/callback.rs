use parser::HttpParser;

#[derive(Copy)]
pub enum CallbackDecision {
    Nothing,
    SkipBody,
}

pub type CallbackResult = Result<CallbackDecision, String>;

pub trait HttpParserCallback {
    #[allow(unused_variables)]
    fn on_message_begin(&mut self, parser: &mut HttpParser) -> CallbackResult {
        Ok(CallbackDecision::Nothing)
    }

    #[allow(unused_variables)]
    fn on_url(&mut self, parser: &mut HttpParser, data: &[u8],) -> CallbackResult {
        Ok(CallbackDecision::Nothing)
    }

    #[allow(unused_variables)]
    fn on_status(&mut self, parser: &mut HttpParser, data: &[u8]) -> CallbackResult {
        Ok(CallbackDecision::Nothing)
    }

    #[allow(unused_variables)]
    fn on_header_field(&mut self, parser: &mut HttpParser, data: &[u8]) -> CallbackResult {
        Ok(CallbackDecision::Nothing)
    }

    #[allow(unused_variables)]
    fn on_header_value(&mut self, parser: &mut HttpParser, data: &[u8]) -> CallbackResult {
        Ok(CallbackDecision::Nothing)
    }

    #[allow(unused_variables)]
    fn on_headers_complete(&mut self, parser: &mut HttpParser) -> CallbackResult {
        Ok(CallbackDecision::Nothing)
    }

    #[allow(unused_variables)]
    fn on_body(&mut self, parser: &mut HttpParser, data: &[u8]) -> CallbackResult {
        Ok(CallbackDecision::Nothing)
    }

    #[allow(unused_variables)]
    fn on_message_complete(&mut self, parser: &mut HttpParser) -> CallbackResult {
        Ok(CallbackDecision::Nothing)
    }
}
