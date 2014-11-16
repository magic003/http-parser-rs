#![warn(experimental)]
#![feature(macro_rules)]

extern crate collections;

use std::u64;
use std::collections::Bitv;
use collections::bitv;

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
    index : uint,             // index into current matcher

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

    fn on_url(&self, buff : &[u8], start : uint, length : uint) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_status(&self, buff : &[u8], start : uint, length : uint) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_header_field(&self, buff : &[u8], start : uint, length : uint) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_header_value(&self, buff : &[u8], start : uint, length : uint) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_headers_complete(&self) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_body(&self, buff : &[u8], start : uint, length : uint) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_message_complete(&self) -> Result<i8, &str> {
        Ok(0)
    }
}

//============== End of public interfaces ===================

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

macro_rules! callback_data(
    ($parser:ident, $mark:ident, $cb:expr, $err:expr, $idx:expr) => (
        if $mark.is_some() {
            match $cb {
                Err(..) => $parser.errno = $err,
                _ => (),
            }

            if $parser.errno != error::Ok {
                return $idx;
            }
            $mark = None;
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
        if $mark.is_none() {
            $mark = Some($idx);
        }
    );
)

const HTTP_MAX_HEADER_SIZE : u32 = 80*1024;
const ULLONG_MAX : u64 = u64::MAX - 1;

const HTTP_PARSER_STRICT : bool = true;

const CR : u8 = b'\r';
const LF : u8 = b'\n';

const normal_url_char : [u8, ..32] = [
    //   0 nul    1 soh    2 stx    3 etx    4 eot    5 enq    6 ack    7 bel   
            0    |   0    |   0    |   0    |   0    |   0    |   0    |   0,       
    //   8 bs     9 ht    10 nl    11 vt    12 np    13 cr    14 so    15 si
            0    |   2    |   0    |   0    |   16   |   0    |   0    |   0, // TODO add T()
    //  16 dle   17 dc1   18 dc2   19 dc3   20 dc4   21 nak   22 syn   23 etb
            0    |   0    |   0    |   0    |   0    |   0    |   0    |   0,       
    //  24 can   25 em    26 sub   27 esc   28 fs    29 gs    30 rs    31 us    
            0    |   0    |   0    |   0    |   0    |   0    |   0    |   0,       
    //  32 sp    33  !    34  "    35  #    36  $    37  %    38  &    39  '    
            0    |   2    |   4    |   0    |   16   |   32   |   64   |  128,      
    //  40  (    41  )    42  *    43  +    44  ,    45  -    46  .    47  /    
            1    |   2    |   4    |   8    |   16   |   32   |   64   |  128,      
    //  48  0    49  1    50  2    51  3    52  4    53  5    54  6    55  7    
            1    |   2    |   4    |   8    |   16   |   32   |   64   |  128,      
    //  56  8    57  9    58  :    59  ;    60  <    61  =    62  >    63  ?    
            1    |   2    |   4    |   8    |   16   |   32   |   64   |   0,       
    //  64  @    65  A    66  B    67  C    68  D    69  E    70  F    71  G    
            1    |   2    |   4    |   8    |   16   |   32   |   64   |  128,      
    //  72  H    73  I    74  J    75  K    76  L    77  M    78  N    79  O    
            1    |   2    |   4    |   8    |   16   |   32   |   64   |  128,      
    //  80  P    81  Q    82  R    83  S    84  T    85  U    86  V    87  W    
            1    |   2    |   4    |   8    |   16   |   32   |   64   |  128,      
    //  88  X    89  Y    90  Z    91  [    92  \    93  ]    94  ^    95  _    
            1    |   2    |   4    |   8    |   16   |   32   |   64   |  128,      
    //  96  `    97  a    98  b    99  c   100  d   101  e   102  f   103  g    
            1    |   2    |   4    |   8    |   16   |   32   |   64   |  128,      
    // 104  h   105  i   106  j   107  k   108  l   109  m   110  n   111  o   
            1    |   2    |   4    |   8    |   16   |   32   |   64   |  128,      
    // 112  p   113  q   114  r   115  s   116  t   117  u   118  v   119  w   
            1    |   2    |   4    |   8    |   16   |   32   |   64   |  128,      
    // 120  x   121  y   122  z   123  {   124  |   125  }   126  ~   127 del
            1    |   2    |   4    |   8    |   16   |   32   |   64   |   0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn is_url_char(ch : u8) -> bool {
    let res = (normal_url_char[(ch >> 3) as uint] & (1 << ((ch & 7) as uint))) != 0;
    res || (!HTTP_PARSER_STRICT && (ch & 0x80) > 0)
}

fn is_num(ch : u8) -> bool {
    ch >= b'0' && ch <= b'9'
}

fn is_alpha(ch : u8) -> bool {
    (ch >= b'a' && ch <= b'z') || (ch >= b'A' && ch <= b'Z')
}

fn is_alphanum(ch : u8) -> bool {
    is_num(ch) || is_alpha(ch)
}

fn is_mark(ch : u8) -> bool {
    ch == b'-' || ch == b'_' || ch == b'.' || ch == b'!' || ch == b'~' || 
        ch == b'*' || ch == b'\'' || ch == b'(' || ch == b')'
}

fn is_userinfo_char(ch : u8) -> bool {
    is_alphanum(ch) || is_mark(ch) || ch == b'%' || 
        ch == b';' || ch == b':' || ch == b'&' || ch == b'=' || 
        ch == b'+' || ch == b'$' || ch == b','
}

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

    pub fn execute(&mut self, cb : T, data : &[u8]) -> uint {
        let mut index : uint = 0;
        let mut header_field_mark : Option<uint> = None;
        let mut header_value_mark : Option<uint> = None;
        let mut url_mark : Option<uint> = None;
        let mut status_mark : Option<uint> = None;

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
            header_field_mark = Some(0);
        }
        if self.state == state::HeaderValue {
            header_value_mark = Some(0);
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
            state::ReqFragment => url_mark = Some(0),
            state::ResStatus => status_mark = Some(0),
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
                            if !is_num(ch) {
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
                        if !is_num(ch) {
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
                            if !is_num(ch) {
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
                        if !is_num(ch) {
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
                        if !is_num(ch) {
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
                            callback_data!(self, status_mark,
                                cb.on_status(data, status_mark.unwrap(), index - status_mark.unwrap()),
                                error::CBStatus, index);
                        } else if ch == LF {
                            self.state = state::HeaderFieldStart;
                            callback_data!(self, status_mark,
                                cb.on_status(data, status_mark.unwrap(), index - status_mark.unwrap()),
                                error::CBStatus, index);
                        }
                    },
                    state::ResLineAlmostDone => {
                        strict_check!(self, ch != LF, index);
                        self.state = state::HeaderFieldStart;
                    },
                    state::StartReq => {
                        if ch != CR && ch != LF {
                            self.flags = 0;
                            self.content_length = ULLONG_MAX;

                            if !is_alpha(ch) {
                                self.errno = error::InvalidMethod;
                                return index;
                            }

                            self.method = http_method::Delete;
                            self.index = 1;
                            match ch {
                                b'C' => self.method = http_method::Connect, // or Copy, Checkout
                                b'D' => self.method = http_method::Delete,
                                b'G' => self.method = http_method::Get,
                                b'H' => self.method = http_method::Head,
                                b'L' => self.method = http_method::Lock,
                                b'M' => self.method = http_method::MKCol, // or Move, MKActivity, Merge, MSearch, MKCalendar
                                b'N' => self.method = http_method::Notify,
                                b'O' => self.method = http_method::Options,
                                b'P' => self.method = http_method::Post, // or PropFind|PropPatch|Put|Patch|Purge
                                b'R' => self.method = http_method::Report,
                                b'S' => self.method = http_method::Subscribe, // or Search
                                b'T' => self.method = http_method::Trace,
                                b'U' => self.method = http_method::Unlock, // or Unsubscribe
                                _ => {
                                    self.errno = error::InvalidMethod;
                                    return index;
                                },
                            }
                            self.state = state::ReqMethod;

                            callback!(self, cb.on_message_begin(), 
                                      error::CBMessageBegin);
                            if self.errno == error::Ok {
                                return index;
                            }
                        }
                    },
                    state::ReqMethod => {
                        if index == data.len() {
                            self.errno = error::InvalidMethod;
                            return index;
                        }

                        let matcher_string = self.method.to_string();
                        let matcher = matcher_string.as_slice();
                        if ch == b' ' && self.index == matcher.len() {
                            self.state = state::ReqSpacesBeforeUrl;
                        } else if ch == (matcher.char_at(self.index) as u8) {
                            ;
                        } else if self.method == http_method::Connect {
                            if self.index == 1 && ch == b'H' {
                                self.method = http_method::Checkout;
                            } else if self.index == 2 && ch == b'P' {
                                self.method = http_method::Copy;
                            } else {
                                self.errno = error::InvalidMethod;
                                return index;
                            }
                        } else if self.method == http_method::MKCol {
                            if self.index == 1 && ch == b'O' {
                                self.method = http_method::Move;
                            } else if self.index == 1 && ch == b'E' {
                                self.method = http_method::Merge;
                            } else if self.index == 1 && ch == b'-' {
                                self.method = http_method::MSearch;
                            } else if self.index == 2 && ch == b'A' {
                                self.method = http_method::MKActivity;
                            } else if self.index == 3 && ch == b'A' {
                                self.method = http_method::MKCalendar;
                            } else {
                                self.errno = error::InvalidMethod;
                                return index;
                            }
                        } else if self.method == http_method::Subscribe {
                            if self.index == 1 && ch == b'E' {
                                self.method = http_method::Search;
                            } else {
                                self.errno == error::InvalidMethod;
                                return index;
                            }
                        } else if self.index == 1 && self.method == http_method::Post {
                           if ch == b'R' {
                               self.method = http_method::PropFind; // or PropPatch
                           } else if ch == b'U' {
                               self.method = http_method::Put; // or Purge
                           } else if ch == b'A' {
                               self.method = http_method::Patch;
                           } else {
                               self.errno = error::InvalidMethod;
                               return index;
                           }
                        } else if self.index == 2 {
                            if self.method == http_method::Put {
                                if ch == b'R' {
                                    self.method = http_method::Purge;
                                } else {
                                    self.errno = error::InvalidMethod;
                                    return index;
                                }
                            } else if self.method == http_method::Unlock {
                                if ch == b'S' {
                                    self.method = http_method::Unsubscribe;
                                } else {
                                    self.errno = error::InvalidMethod;
                                    return index;
                                }
                            } else {
                                self.errno = error::InvalidMethod;
                                return index;
                            }
                        } else if self.index == 4 && self.method == http_method::PropFind && ch == b'P' {
                            self.method = http_method::PropPatch;
                        } else {
                            self.errno = error::InvalidMethod;
                            return index;
                        }

                        self.index += 1;
                    },
                    state::ReqSpacesBeforeUrl => {
                        if ch != b' ' {
                            mark!(url_mark, index);
                            if self.method == http_method::Connect {
                                self.state = state::ReqServerStart;
                            }

                            self.state = HttpParser::parse_url_char(self.state, ch);
                            if (self.state == state::Dead) {
                                self.errno = error::InvalidUrl;
                                return index;
                            }
                        }
                    },
                    state::ReqSchema |
                    state::ReqSchemaSlash |
                    state::ReqSchemaSlashSlash |
                    state::ReqServerStart => {
                        match ch {
                            // No whitespace allowed here
                            b' ' | CR | LF => {
                                self.errno = error::InvalidUrl;
                                return index;
                            },
                            _ => {
                                self.state = HttpParser::parse_url_char(self.state, ch);
                                if self.state == state::Dead {
                                    self.errno = error::InvalidUrl;
                                    return index;
                                }
                            }
                        }
                    },
                    state::ReqServer |
                    state::ReqServerWithAt |
                    state::ReqPath |
                    state::ReqQueryStringStart |
                    state::ReqQueryString |
                    state::ReqFragmentStart |
                    state::ReqFragment => {
                        match ch {
                            b' ' => {
                                self.state = state::ReqHttpStart;
                                callback_data!(self, url_mark,
                                    cb.on_status(data, url_mark.unwrap(), index - url_mark.unwrap()),
                                    error::CBUrl, index);
                            },
                            CR | LF => {
                                self.http_major = 0;
                                self.http_minor = 9;
                                self.state = if ch == CR {
                                    state::ReqLineAlmostDone 
                                } else {
                                    state::HeaderFieldStart
                                };
                                callback_data!(self, url_mark,
                                    cb.on_status(data, url_mark.unwrap(), index - url_mark.unwrap()),
                                    error::CBUrl, index);
                            },
                            _ => {
                                self.state = HttpParser::parse_url_char(self.state, ch);
                                if self.state == state::Dead {
                                    self.errno = error::InvalidUrl;
                                    return index;
                                }
                            }
                        }
                    },
                    state::ReqHttpStart => {
                    }
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


    // Our URL parser
    fn parse_url_char(s : state::State, ch : u8) -> state::State {
        if ch == b' ' || ch == b'\r' || ch == b'\n' {
            return state::Dead;
        }

        if HTTP_PARSER_STRICT {
            if ch == b'\t' || ch == b'\x0C' {   // '\x0C' = '\f' 
                return state::Dead;
            }
        }

        match s {
            state::ReqSpacesBeforeUrl => {
                // Proxied requests are followed by scheme of an absolute URI (alpha).
                // All methods except CONNECT are followed by '/' or '*'.

                if ch == b'/' || ch == b'*' {
                    return state::ReqPath;
                }

                if is_alpha(ch) {
                    return state::ReqSchema;
                }
            },
            state::ReqSchema => {
                if is_alpha(ch) {
                    return s;
                }

                if ch == b':' {
                    return state::ReqSchemaSlash;
                }
            },
            state::ReqSchemaSlash => {
                if ch == b'/' {
                    return state::ReqSchemaSlashSlash;
                }
            },
            state::ReqSchemaSlashSlash => {
                if ch == b'/' {
                    return state::ReqServerStart;
                }
            },
            state::ReqServerWithAt if ch == b'@' => return state::Dead,
            state::ReqServerWithAt | state::ReqServerStart | state::ReqServer => {
                if ch == b'/' {
                    return state::ReqPath;
                }

                if ch == b'?' {
                    return state::ReqQueryStringStart;
                }

                if ch == b'@' {
                    return state::ReqServerWithAt;
                }

                if is_userinfo_char(ch) || ch == b'[' || ch == b']' {
                    return state::ReqServer;
                }
            },
            state::ReqPath => {
                if is_url_char(ch) {
                    return s;
                }

                match ch {
                    b'?' => return state::ReqQueryStringStart,
                    b'#' => return state::ReqFragmentStart,
                    _    => (),
                }
            },
            state::ReqQueryStringStart | state::ReqQueryString => {
                if is_url_char(ch) {
                    return state::ReqQueryString;
                }

                match ch {
                    b'?' => return state::ReqQueryString, // allow extra '?' in query string
                    b'#' => return state::ReqFragmentStart,
                    _    => (),
                }
            },
            state::ReqFragmentStart => {
                if is_url_char(ch) {
                    return state::ReqFragment;
                }

                match ch {
                    b'?' => return state::ReqFragment,
                    b'#' => return s,
                    _    => (),
                }
            },
            state::ReqFragment => {
                if is_url_char(ch) {
                    return s;
                }

                if ch == b'?' || ch == b'#' {
                    return s;
                }
            },
            _ => (),
        }

        // We should never fall out of the switch above unless there's an error
        return state::Dead;
    }
}
