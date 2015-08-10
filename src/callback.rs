use parser::HttpParser;

/// `ParseAction` defines the potential actions that could be returned by any callback function.
/// The parser uses it to determine consequent behavior.
#[derive(Clone)]
pub enum ParseAction {
    /// No special actions. Keep the normal execution.
    None,
    /// Skip body
    SkipBody,
}

/// Result of a callback function.
pub type CallbackResult = Result<ParseAction, String>;

/// It defines the callback functions that would be called by parser.
///
/// # Example
///
/// ```
/// # use http_parser::*;
/// #
/// struct Callback;
///
/// impl HttpParserCallback for Callback {
///     fn on_message_begin(&mut self, parser: &mut HttpParser) -> CallbackResult {
///         println!("Message begin");
///         Ok(ParseAction::None)
///     }
///
///     // Override other functions as you wish
/// }
///
/// let mut cb = Callback;
/// ```
pub trait HttpParserCallback {
    /// Function called when starting parsing a new HTTP request or response.
    #[allow(unused_variables)]
    fn on_message_begin(&mut self, parser: &mut HttpParser) -> CallbackResult {
        Ok(ParseAction::None)
    }

    /// Function called when a URL is parsed.
    #[allow(unused_variables)]
    fn on_url(&mut self, parser: &mut HttpParser, data: &[u8],) -> CallbackResult {
        Ok(ParseAction::None)
    }

    /// Function called when a status is parsed.
    #[allow(unused_variables)]
    fn on_status(&mut self, parser: &mut HttpParser, data: &[u8]) -> CallbackResult {
        Ok(ParseAction::None)
    }

    /// Function called when a header field is parsed.
    #[allow(unused_variables)]
    fn on_header_field(&mut self, parser: &mut HttpParser, data: &[u8]) -> CallbackResult {
        Ok(ParseAction::None)
    }

    /// Function called when a header value is parsed.
    #[allow(unused_variables)]
    fn on_header_value(&mut self, parser: &mut HttpParser, data: &[u8]) -> CallbackResult {
        Ok(ParseAction::None)
    }

    /// Function called when all headers are parsed.
    #[allow(unused_variables)]
    fn on_headers_complete(&mut self, parser: &mut HttpParser) -> CallbackResult {
        Ok(ParseAction::None)
    }

    /// Function called when the body is parsed.
    #[allow(unused_variables)]
    fn on_body(&mut self, parser: &mut HttpParser, data: &[u8]) -> CallbackResult {
        Ok(ParseAction::None)
    }

    /// Function called when finishing parsing a HTTP request or response.
    #[allow(unused_variables)]
    fn on_message_complete(&mut self, parser: &mut HttpParser) -> CallbackResult {
        Ok(ParseAction::None)
    }
}
