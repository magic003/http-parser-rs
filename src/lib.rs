#![warn(experimental)]

mod error;
mod state;

pub enum HttpParserType {
    HttpRequest,
    HttpResponse,
    HttpBoth,
}

pub struct HttpParser {
    tp : HttpParserType,
    errno : error::HttpErrno,
    state : state::State,
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

impl<T: HttpParserCallback> HttpParser {
    pub fn new(tp : HttpParserType) -> HttpParser {
        HttpParser { 
            tp : tp,  
            errno : error::Ok,
            state : match tp {
                        HttpRequest     => state::StartReq,
                        HttpResponse    => state::StartRes,
                        HttpBoth        => state::StartReqOrRes,
                    },
        }
    }

    pub fn execute(&self, cb : T) -> int {
        0
    }
}
