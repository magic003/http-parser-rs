#![warn(experimental)]
#![feature(macro_rules)]

use std::u64;

mod error;
mod state;
mod flags;
mod http_method;

pub enum HttpParserType {
    HttpRequest,
    HttpResponse,
    HttpBoth,
}

pub struct HttpParser {
    // private
    tp : HttpParserType,
    state : state::State,
    flags : u8,
    index : u8,             // index into current matcher

    nread : u32,            // bytes read in various scenarios
    content_length : u64,   // bytes in body (0 if no Content-Length header
    
    // read-only
    http_major : u8,
    http_minor : u8,
    errno : error::HttpErrno,
    status_code : u16,                          // response only
    method : http_method::HttpMethod,            // request only
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
const ULLONG_MAX : u64 = u64::MAX - 1;

const HTTP_PARSER_STRICT : bool = true;

const CR : u8 = b'\r';
const LF : u8 = b'\n';

macro_rules! ensure_error(
    ($parser:ident) => (
        if $parser.errno == error::Ok {
            $parser.errno = error::Unknown;
        }
    );
)

macro_rules! assert_ok(
    ($parser:ident) => (
        assert!($parser.errno == error::Ok);
    );
)

macro_rules! callback(
    ($parser:ident, $cb:expr, $err:expr) => (
       match $cb {
           Err(..) => $parser.errno = $err,
           _ => (),
       }
    );
)

macro_rules! strict_check(
    ($parser:ident, $cond:expr, $idx:expr) => (
        if HTTP_PARSER_STRICT && $cond {
            $parser.errno = error::Strict;
            return $idx;
        }
    );
)

macro_rules! mark(
    ($mark:ident, $idx:expr) => (
        if $mark != 0 {
            $mark = $idx;
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
            flags : 0,
            index : 0,
            nread : 0,
            content_length: ULLONG_MAX,
            http_major: 1,
            http_minor: 0,
            errno : error::Ok,
            status_code : 0,
            method : http_method::Get,
        }
    }

    pub fn execute(&mut self, cb : T, data : &[u8]) -> int {
        let mut index = 0i;
        let mut header_field_mark = 0i;
        let mut header_value_mark = 0i;
        let mut url_mark = 0i;
        let mut status_mark = 0i;

        if self.errno == error::Ok {
            return 0;
        }

        if data.len() == 0 {    // mean EOF
            match self.state {
                state::BodyIdentityEof => {
                    assert_ok!(self);
                    callback!(self, cb.on_message_complete(), 
                              error::CBMessageComplete);
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

        for &ch in data.iter() {
            if self.state <= state::HeadersDone {
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
                if self.nread > HTTP_MAX_HEADER_SIZE {
                    self.errno = error::HeaderOverflow;
                    return index;
                }
            }

            // using loop to mimic 'goto reexecute_byte' in http_parser.c
            let mut retry = false;
            loop {
                match self.state {
                    state::Dead => {
                        if ch != CR && ch != LF {
                            self.errno = error::ClosedConnection;
                            return index;
                        }
                    },
                    state::StartReqOrRes => {
                        if ch != CR && ch != LF {
                            self.flags = 0;
                            self.content_length = ULLONG_MAX;

                            if ch == b'H' {
                                self.state = state::ResOrRespH;
                                assert_ok!(self);
                                callback!(self, cb.on_message_begin(),
                                    error::CBMessageBegin);
                                if self.errno != error::Ok {
                                    return index+1;
                                }
                            } else {
                                self.tp = HttpRequest;
                                self.state = state::StartReq;
                                retry = true;
                            }
                        }
                    },
                    state::ResOrRespH => {
                        if ch == b'T' {
                            self.tp = HttpResponse;
                            self.state = state::ResHT;
                        } else {
                            if ch != b'E' {
                                self.errno = error::InvalidConstant;
                                return index;
                            }

                            self.tp = HttpRequest;
                            self.method = http_method::Head;
                            self.index = 2;
                            self.state = state::ReqMethod;
                        }
                    },
                    state::StartRes => {
                        self.flags = 0;
                        self.content_length = ULLONG_MAX;

                        match ch {
                            b'H' => self.state = state::ResH,
                            CR | LF => (),
                            _ => {
                                self.errno = error::InvalidConstant;
                                return index;
                            }
                        }
                        
                        assert_ok!(self);
                        callback!(self, cb.on_message_begin(), 
                                  error::CBMessageBegin);
                    },
                    state::ResH => {
                        strict_check!(self, ch != b'T', index);                       
                        self.state = state::ResHT;
                    },
                    state::ResHT => {
                        strict_check!(self, ch != b'T', index);
                        self.state = state::ResHTT;
                    },
                    state::ResHTT => {
                        strict_check!(self, ch != b'P', index);
                        self.state = state::ResHTTP;
                    },
                    state::ResHTTP => {
                        strict_check!(self, ch != b'/', index);
                        self.state = state::ResFirstHttpMajor;
                    },
                    state::ResFirstHttpMajor => {
                        if ch < b'0' || ch > b'9' {
                            self.errno = error::InvalidVersion;
                            return index;
                        }
                        self.http_major = ch - b'0';
                        self.state = state::ResHttpMajor;
                    },
                    state::ResHttpMajor => {
                        if ch == b'.' {
                            self.state = state::ResFirstHttpMinor;
                        } else {
                            if !HttpParser::is_num(ch) {
                                self.errno = error::InvalidVersion;
                                return index;
                            }

                            self.http_major *= 10;
                            self.http_major += ch - b'0';

                            if self.http_major > 999 {
                                self.errno = error::InvalidVersion;
                                return index;
                            }
                        }
                    },
                    state::ResFirstHttpMinor => {
                        if !HttpParser::is_num(ch) {
                            self.errno = error::InvalidVersion;
                            return index;
                        }

                        self.http_minor = ch - b'0';
                        self.state = state::ResHttpMinor;
                    },
                    // minor HTTP version or end of request line
                    state::ResHttpMinor => {
                        if ch == b' ' {
                            self.state = state::ResFirstStatusCode;
                        } else {
                            if !HttpParser::is_num(ch) {
                                self.errno = error::InvalidVersion;
                                return index;
                            }

                            self.http_minor *= 10;
                            self.http_minor += ch - b'0';

                            if self.http_minor > 999 {
                                self.errno = error::InvalidVersion;
                                return index;
                            }
                        }
                    },
                    state::ResFirstStatusCode => {
                        if !HttpParser::is_num(ch) {
                            if ch != b' ' {
                                self.errno = error::InvalidStatus;
                                return index;
                            }
                        } else {
                            self.status_code = (ch - b'0') as u16;
                            self.state = state::ResStatusCode;
                        }
                    },
                    state::ResStatusCode => {
                        if !HttpParser::is_num(ch) {
                            match ch {
                                b' ' => self.state = state::ResStatusStart,
                                CR   => self.state = state::ResLineAlmostDone,
                                LF   => self.state = state::HeaderFieldStart,
                                _    => {
                                    self.errno = error::InvalidStatus;
                                    return index;
                                }
                            }
                        }

                        self.status_code *= 10;
                        self.status_code += (ch - b'0') as u16;

                        if self.status_code > 999 {
                            self.errno = error::InvalidStatus;
                            return index;
                        }
                    },
                    state::ResStatusStart => {
                        if ch == CR {
                            self.state = state::ResLineAlmostDone;
                        } else if ch == LF {
                            self.state = state::HeaderFieldStart;
                        } else {
                            mark!(status_mark, index);
                            self.state = state::ResStatus;
                            self.index = 0;
                        }
                    },
                    state::ResStatus => {
                        if ch == CR {
                            self.state = state::ResLineAlmostDone;
                        } else if ch == LF {
                            self.state = state::HeaderFieldStart;
                        }
                    },
                    _ => (),
                }

                if !retry {
                    break;
                }
            }
        }

        index += 1;
        0
    }

    fn is_num(ch : u8) -> bool {
        ch >= b'0' && ch <= b'9'
    }
}
