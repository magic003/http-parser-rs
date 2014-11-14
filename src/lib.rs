#![warn(experimental)]
#![feature(macro_rules)]

mod error;
mod state;

pub enum HttpParserType {
    HttpRequest,
    HttpResponse,
    HttpBoth,
}

pub struct HttpParser {
    // private
    tp : HttpParserType,
    state : state::State,
    nread : u32,            // bytes read in various scenarios
    
    // read-only
    errno : error::HttpErrno,
}

pub trait HttpParserCallback {
    fn on_message_begin(&self) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_url(&self) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_status(&self) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_header_field(&self) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_headers_complete(&self) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_body(&self) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_message_complete(&self) -> Result<i8, &str> {
        Ok(0)
    }
}

const HTTP_MAX_HEADER_SIZE : u32 = 80*1024;

macro_rules! unknown_error(
    ($parser:ident) => (
        if $parser.errno == error::Ok {
            $parser.errno = error::Unknown;
        }
    );
)

impl<T: HttpParserCallback> HttpParser {
    pub fn new(tp : HttpParserType) -> HttpParser {
        HttpParser { 
            tp : tp,  
            state : match tp {
                        HttpRequest     => state::StartReq,
                        HttpResponse    => state::StartRes,
                        HttpBoth        => state::StartReqOrRes,
                    },
            nread : 0,
            errno : error::Ok,
        }
    }

    pub fn execute(&mut self, cb : T, data : &[u8]) -> int {
        let mut index = 0;
        let mut header_field_mark = 0u8;
        let mut header_value_mark = 0u8;
        let mut url_mark = 0u8;
        let mut status_mark = 0u8;

        if self.errno == error::Ok {
            return 0;
        }

        if data.len() == 0 {    // mean EOF
            match self.state {
                state::BodyIdentityEof => {
                    assert!(self.errno == error::Ok);
                    match cb.on_message_complete() {
                        Err(..) => self.errno = error::CBMessageComplete,
                        _ => (),
                    }

                    if self.errno == error::Ok {
                        return index;
                    }
                    return 0;
                },
                state::Dead | 
                state::StartReqOrRes | 
                state::StartReq | 
                state::StartRes => {
                    return 0;
                }
                _ => {
                   self.errno = error::InvalidEofState;
                   return 1;
                }
            }
        }

        if self.state == state::HeaderField {
            header_field_mark = 1;
        }
        if self.state == state::HeaderValue {
            header_value_mark = 1;
        }
        match self.state {
            state::ReqPath |
            state::ReqSchema |
            state::ReqSchemaSlash |
            state::ReqSchemaSlashSlash |
            state::ReqServerStart |
            state::ReqServer |
            state::ReqServerWithAt |
            state::ReqQueryStringStart |
            state::ReqQueryString |
            state::ReqFragmentStart |
            state::ReqFragment => url_mark = 1,
            state::ResStatus => status_mark = 1,
            _ => (),
        }

        for ch in data.iter() {
            if (self.state <= state::HeadersDone) {
                self.nread += 1;

                // From http_parser.c

                // Don't allow the total size of the HTTP headers (including the status
                // line) to exceed HTTP_MAX_HEADER_SIZE. This check is here to protect
                // embedders against denial-of-service attacks where the attacker feeds
                // us a never-ending header that the embedder keeps buffering.
                // 
                // This check is arguably the responsibility of embedders but we're doing
                // it on the embedder's behalf because most won't bother and this way we
                // make the web a little safer. HTTP_MAX_HEADER_SIZE is still far bigger
                // than any reasonable request or response so this should never affect
                // day-to-day operation.
                if (self.nread > HTTP_MAX_HEADER_SIZE) {
                    self.errno = error::HeaderOverflow;
                    unknown_error!(self);
                    return index;
                }
            }
        }

        // using loop to mimic 'goto reexecute_byte' in http_parser.c
        let mut retry = false;
        loop {

        }
        0
    }
}
