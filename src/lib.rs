#![warn(experimental)]
#![feature(macro_rules)]
#![allow(experimental)]

extern crate collections;

use std::u64;
use std::cmp;

// for tests
use std::str;
use std::default::Default;

pub use self::error::HttpErrno;
pub use self::http_method::HttpMethod;

mod error;
mod state;
mod flags;
mod http_method;


#[deriving(PartialEq, Eq, Copy)]
pub enum HttpParserType {
    HttpRequest,
    HttpResponse,
    HttpBoth,
}

pub struct HttpParser {
    // private
    tp : HttpParserType,
    state : state::State,
    header_state : state::HeaderState,
    flags : u8,
    index : uint,             // index into current matcher

    nread : u32,            // bytes read in various scenarios
    content_length : u64,   // bytes in body (0 if no Content-Length header
    
    // read-only
    pub http_major : u8,
    pub http_minor : u8,
    pub errno : error::HttpErrno,
    pub status_code : u16,                          // response only
    pub method : http_method::HttpMethod,            // request only

    pub upgrade : bool,
}

pub trait HttpParserCallback {
    fn on_message_begin(&mut self, parser : &HttpParser) -> Result<i8, &str> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn on_url(&mut self, parser : &HttpParser, data : &[u8],) -> Result<i8, &str> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn on_status(&mut self, parser : &HttpParser, data : &[u8]) -> Result<i8, &str> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn on_header_field(&mut self, parser : &HttpParser, data : &[u8]) -> Result<i8, &str> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn on_header_value(&mut self, parser : &HttpParser, data : &[u8]) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_headers_complete(&mut self, parser : &HttpParser) -> Result<i8, &str> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn on_body(&mut self, parser : &HttpParser, data : &[u8]) -> Result<i8, &str> {
        Ok(0)
    }

    fn on_message_complete(&mut self, parser : &HttpParser) -> Result<i8, &str> {
        Ok(0)
    }
}

//============== End of public interfaces ===================

macro_rules! ensure_error(
    ($parser:ident) => (
        if $parser.errno == error::HttpErrno::Ok {
            $parser.errno = error::HttpErrno::Unknown;
        }
    );
);

macro_rules! assert_ok(
    ($parser:ident) => (
        assert!($parser.errno == error::HttpErrno::Ok);
    );
);

macro_rules! callback(
    ($parser:ident, $cb:expr, $err:expr) => (
       match $cb {
           Err(..) => $parser.errno = $err,
           _ => (),
       }
    );
);

macro_rules! callback_data(
    ($parser:ident, $mark:ident, $cb:expr, $err:expr, $idx:expr) => (
        if $mark.is_some() {
            match $cb {
                Err(..) => $parser.errno = $err,
                _ => (),
            }

            if $parser.errno != error::HttpErrno::Ok {
                return $idx;
            }
            // Necessary to reset mark, though it causes unused warning
            $mark = None;
        }
    );
);

macro_rules! start_state(
    ($parser:ident) => (
        if $parser.tp == HttpParserType::HttpRequest {
            state::State::StartReq
        } else {
            state::State::StartRes
        }
    );
);

macro_rules! strict_check(
    ($parser:ident, $cond:expr, $idx:expr) => (
        if HTTP_PARSER_STRICT && $cond {
            $parser.errno = error::HttpErrno::Strict;
            return $idx;
        }
    );
);

macro_rules! new_message(
    ($parser:ident) => (
        if HTTP_PARSER_STRICT {
            if $parser.http_should_keep_alive() {
                start_state!($parser)
            } else {
                state::State::Dead
            }
        } else {
            start_state!($parser)
        }
    );
);

macro_rules! mark(
    ($mark:ident, $idx:expr) => (
        if $mark.is_none() {
            $mark = Some($idx);
        }
    );
);

const HTTP_MAX_HEADER_SIZE : u32 = 80*1024;
const ULLONG_MAX : u64 = u64::MAX - 1;

const HTTP_PARSER_STRICT : bool = true;

const CR : u8 = b'\r';
const LF : u8 = b'\n';

const PROXY_CONNECTION : &'static str = "proxy-connection";
const CONNECTION : &'static str = "connection";
const CONTENT_LENGTH : &'static str = "content-length";
const TRANSFER_ENCODING : &'static str = "transfer-encoding";
const UPGRADE : &'static str = "upgrade";
const CHUNKED : &'static str = "chunked";
const KEEP_ALIVE : &'static str = "keep-alive";
const CLOSE : &'static str = "close";

const TOKEN : [Option<u8>, ..256] = [
    //   0 nul      1 soh       2 stx       3 etx      4 eot        5 enq       6 ack       7 bel   
         None,       None,     None,        None,       None,       None,        None,      None,       
    //   8 bs        9 ht      10 nl        11 vt      12 np        13 cr       14 so       15 si    
         None,       None,     None,        None,       None,       None,        None,      None,       
    //  16 dle     17 dc1      18 dc2       19 dc3     20 dc4       21 nak      22 syn      23 etb    
         None,       None,     None,        None,       None,       None,        None,      None,       
    //  24 can     25 em       26 sub       27 esc     28 fs        29 gs       30 rs       31 us    
         None,       None,     None,        None,       None,       None,        None,      None,       
    //  32 sp      33   !      34  "        35  #      36   $       37  %       38  &       39   '    
         None, Some(b'!'),     None,  Some(b'#'), Some(b'$'),  Some(b'%'), Some(b'&'), Some(b'\''),      
    //  40  (      41  )       42  *        43  +      44  ,        45  -       46  .       47  /    
         None,     None, Some(b'*'),  Some(b'+'),      None,  Some(b'-'), Some(b'.'),       None,       
    //  48  0      49  1       50  2        51  3      52  4        53  5       54  6       55  7    
       Some(b'0'), Some(b'1'), Some(b'2'), Some(b'3'), Some(b'4'), Some(b'5'), Some(b'6'), Some(b'7'),      
    //  56  8      57  9       58  :        59  ;      60  <        61  =       62  >       63  ?    
       Some(b'8'), Some(b'9'), None,        None,      None,        None,       None,       None,       
    //  64  @      65  A       66  B        67  C      68  D        69  E       70  F       71  G    
        None, Some(b'a'), Some(b'b'), Some(b'c'), Some(b'd'), Some(b'e'), Some(b'f'), Some(b'g'),      
    //  72  H      73  I       74  J        75  K      76  L        77  M       78  N       79  O    
       Some(b'h'), Some(b'i'), Some(b'j'), Some(b'k'), Some(b'l'), Some(b'm'), Some(b'n'), Some(b'o'),      
    //  80  P      81  Q       82  R        83  S      84  T        85  U       86  V       87  W    
       Some(b'p'), Some(b'q'), Some(b'r'), Some(b's'), Some(b't'), Some(b'u'), Some(b'v'), Some(b'w'),      
    //  88  X      89  Y       90  Z        91  [      92  \        93  ]       94  ^       95  _    
       Some(b'x'), Some(b'y'), Some(b'z'),  None,      None,        None,      Some(b'^'), Some(b'_'),      
    //  96  `      97  a       98  b        99  c      100  d       101  e      102  f      103  g    
       Some(b'`'), Some(b'a'), Some(b'b'),  Some(b'c'), Some(b'd'), Some(b'e'), Some(b'f'), Some(b'g'),      
    // 104  h      105  i      106  j       107  k     108  l       109  m      110  n      111  o    
       Some(b'h'), Some(b'i'), Some(b'j'),  Some(b'k'), Some(b'l'), Some(b'm'), Some(b'n'), Some(b'o'),      
    // 112  p      113  q      114  r       115  s     116  t       117  u      118  v      119  w    
       Some(b'p'), Some(b'q'), Some(b'r'),  Some(b's'), Some(b't'), Some(b'u'), Some(b'v'), Some(b'w'),      
    // 120  x      121  y      122  z       123  {     124  |       125  }      126  ~      127 del    
       Some(b'x'), Some(b'y'), Some(b'z'),  None,       Some(b'|'), None,       Some(b'~'), None,
    // no one is token afterwards
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None,       
        None,       None,     None,        None,       None,       None,        None,      None];

const NORMAL_URL_CHAR : [u8, ..32] = [
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

const UNHEX : [i8, ..256] = [
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
     0, 1, 2, 3, 4, 5, 6, 7, 8, 9,-1,-1,-1,-1,-1,-1,
    -1,10,11,12,13,14,15,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,10,11,12,13,14,15,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1];

// TODO replace some functions by macros

fn token(ch :u8) -> Option<u8> {
    if HTTP_PARSER_STRICT {
        TOKEN[ch as uint]
    } else {
        if ch == b' ' { Some(b' ') } else { TOKEN[ch as uint] }
    }
}

fn is_url_char(ch : u8) -> bool {
    let res = (NORMAL_URL_CHAR[(ch >> 3) as uint] & (1 << ((ch & 7) as uint))) != 0;
    res || (!HTTP_PARSER_STRICT && (ch & 0x80) > 0)
}

fn lower(ch : u8) -> u8 {
    ch | 0x20
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

impl HttpParser {
    pub fn new(tp : HttpParserType) -> HttpParser {
        HttpParser { 
            tp : tp,  
            state : match tp {
                        HttpParserType::HttpRequest     => state::State::StartReq,
                        HttpParserType::HttpResponse    => state::State::StartRes,
                        HttpParserType::HttpBoth        => state::State::StartReqOrRes,
                    },
            header_state : state::HeaderState::General,
            flags : 0,
            index : 0,
            nread : 0,
            content_length: ULLONG_MAX,
            http_major: 1,
            http_minor: 0,
            errno : error::HttpErrno::Ok,
            status_code : 0,
            method : http_method::HttpMethod::Get,
            upgrade : false,
        }
    }

    pub fn execute<T: HttpParserCallback>(&mut self, cb : &mut T, data : &[u8]) -> u64 {
        let mut index : u64 = 0;
        let len : u64 = data.len() as u64;
        let mut header_field_mark : Option<u64> = None;
        let mut header_value_mark : Option<u64> = None;
        let mut url_mark : Option<u64> = None;
        let mut body_mark : Option<u64> = None;
        let mut status_mark : Option<u64> = None;

        if self.errno != error::HttpErrno::Ok {
            return 0;
        }

        if len == 0 {    // mean EOF
            match self.state {
                state::State::BodyIdentityEof => {
                    assert_ok!(self);
                    callback!(self, cb.on_message_complete(self), 
                              error::HttpErrno::CBMessageComplete);
                    if self.errno != error::HttpErrno::Ok {
                        return index;
                    }
                    return 0;
                },
                state::State::Dead | 
                state::State::StartReqOrRes | 
                state::State::StartReq | 
                state::State::StartRes => {
                    return 0;
                }
                _ => {
                   self.errno = error::HttpErrno::InvalidEofState;
                   return 1;
                }
            }
        }

        if self.state == state::State::HeaderField {
            header_field_mark = Some(0);
        }
        if self.state == state::State::HeaderValue {
            header_value_mark = Some(0);
        }
        match self.state {
            state::State::ReqPath |
            state::State::ReqSchema |
            state::State::ReqSchemaSlash |
            state::State::ReqSchemaSlashSlash |
            state::State::ReqServerStart |
            state::State::ReqServer |
            state::State::ReqServerWithAt |
            state::State::ReqQueryStringStart |
            state::State::ReqQueryString |
            state::State::ReqFragmentStart |
            state::State::ReqFragment => url_mark = Some(0),
            state::State::ResStatus => status_mark = Some(0),
            _ => (),
        }

        while index < len {
            let ch = data[index as uint];
            if self.state <= state::State::HeadersDone {
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
                    self.errno = error::HttpErrno::HeaderOverflow;
                    return index;
                }
            }

            // using loop to mimic 'goto reexecute_byte' in http_parser.c
            let mut retry = false;
            loop {
                retry = false;  // reset in each loop
                match self.state {
                    state::State::Dead => {
                        if ch != CR && ch != LF {
                            self.errno = error::HttpErrno::ClosedConnection;
                            return index;
                        }
                    },
                    state::State::StartReqOrRes => {
                        if ch != CR && ch != LF {
                            self.flags = 0;
                            self.content_length = ULLONG_MAX;

                            if ch == b'H' {
                                self.state = state::State::ResOrRespH;
                                assert_ok!(self);
                                callback!(self, cb.on_message_begin(self),
                                    error::HttpErrno::CBMessageBegin);
                                if self.errno != error::HttpErrno::Ok {
                                    return index+1;
                                }
                            } else {
                                self.tp = HttpParserType::HttpRequest;
                                self.state = state::State::StartReq;
                                retry = true;
                            }
                        }
                    },
                    state::State::ResOrRespH => {
                        if ch == b'T' {
                            self.tp = HttpParserType::HttpResponse;
                            self.state = state::State::ResHT;
                        } else {
                            if ch != b'E' {
                                self.errno = error::HttpErrno::InvalidConstant;
                                return index;
                            }

                            self.tp = HttpParserType::HttpRequest;
                            self.method = http_method::HttpMethod::Head;
                            self.index = 2;
                            self.state = state::State::ReqMethod;
                        }
                    },
                    state::State::StartRes => {
                        self.flags = 0;
                        self.content_length = ULLONG_MAX;

                        match ch {
                            b'H' => self.state = state::State::ResH,
                            CR | LF => (),
                            _ => {
                                self.errno = error::HttpErrno::InvalidConstant;
                                return index;
                            },
                        }
                        
                        assert_ok!(self);
                        callback!(self, cb.on_message_begin(self), 
                                  error::HttpErrno::CBMessageBegin);
                        if self.errno != error::HttpErrno::Ok {
                            return index+1;
                        }
                    },
                    state::State::ResH => {
                        strict_check!(self, ch != b'T', index);                       
                        self.state = state::State::ResHT;
                    },
                    state::State::ResHT => {
                        strict_check!(self, ch != b'T', index);
                        self.state = state::State::ResHTT;
                    },
                    state::State::ResHTT => {
                        strict_check!(self, ch != b'P', index);
                        self.state = state::State::ResHTTP;
                    },
                    state::State::ResHTTP => {
                        strict_check!(self, ch != b'/', index);
                        self.state = state::State::ResFirstHttpMajor;
                    },
                    state::State::ResFirstHttpMajor => {
                        if ch < b'0' || ch > b'9' {
                            self.errno = error::HttpErrno::InvalidVersion;
                            return index;
                        }
                        self.http_major = ch - b'0';
                        self.state = state::State::ResHttpMajor;
                    },
                    state::State::ResHttpMajor => {
                        if ch == b'.' {
                            self.state = state::State::ResFirstHttpMinor;
                        } else {
                            if !is_num(ch) {
                                self.errno = error::HttpErrno::InvalidVersion;
                                return index;
                            }

                            self.http_major *= 10;
                            self.http_major += ch - b'0';

                            if self.http_major > 99 {
                                self.errno = error::HttpErrno::InvalidVersion;
                                return index;
                            }
                        }
                    },
                    state::State::ResFirstHttpMinor => {
                        if !is_num(ch) {
                            self.errno = error::HttpErrno::InvalidVersion;
                            return index;
                        }

                        self.http_minor = ch - b'0';
                        self.state = state::State::ResHttpMinor;
                    },
                    // minor HTTP version or end of request line
                    state::State::ResHttpMinor => {
                        if ch == b' ' {
                            self.state = state::State::ResFirstStatusCode;
                        } else {
                            if !is_num(ch) {
                                self.errno = error::HttpErrno::InvalidVersion;
                                return index;
                            }

                            self.http_minor *= 10;
                            self.http_minor += ch - b'0';

                            if self.http_minor > 99 {
                                self.errno = error::HttpErrno::InvalidVersion;
                                return index;
                            }
                        }
                    },
                    state::State::ResFirstStatusCode => {
                        if !is_num(ch) {
                            if ch != b' ' {
                                self.errno = error::HttpErrno::InvalidStatus;
                                return index;
                            }
                        } else {
                            self.status_code = (ch - b'0') as u16;
                            self.state = state::State::ResStatusCode;
                        }
                    },
                    state::State::ResStatusCode => {
                        if !is_num(ch) {
                            match ch {
                                b' ' => self.state = state::State::ResStatusStart,
                                CR   => self.state = state::State::ResLineAlmostDone,
                                LF   => self.state = state::State::HeaderFieldStart,
                                _    => {
                                    self.errno = error::HttpErrno::InvalidStatus;
                                    return index;
                                }
                            }
                        } else {
                            self.status_code *= 10;
                            self.status_code += (ch - b'0') as u16;

                            if self.status_code > 999 {
                                self.errno = error::HttpErrno::InvalidStatus;
                                return index;
                            }
                        }
                    },
                    state::State::ResStatusStart => {
                        if ch == CR {
                            self.state = state::State::ResLineAlmostDone;
                        } else if ch == LF {
                            self.state = state::State::HeaderFieldStart;
                        } else {
                            mark!(status_mark, index);
                            self.state = state::State::ResStatus;
                            self.index = 0;
                        }
                    },
                    state::State::ResStatus => {
                        if ch == CR {
                            self.state = state::State::ResLineAlmostDone;
                            assert_ok!(self);
                            callback_data!(self, status_mark,
                                cb.on_status(self, data.slice(status_mark.unwrap() as uint, index as uint)),
                                error::HttpErrno::CBStatus, index+1);
                        } else if ch == LF {
                            self.state = state::State::HeaderFieldStart;
                            assert_ok!(self);
                            callback_data!(self, status_mark,
                                cb.on_status(self, data.slice(status_mark.unwrap() as uint, index as uint)),
                                error::HttpErrno::CBStatus, index+1);
                        }
                    },
                    state::State::ResLineAlmostDone => {
                        strict_check!(self, ch != LF, index);
                        self.state = state::State::HeaderFieldStart;
                    },
                    state::State::StartReq => {
                        if ch != CR && ch != LF {
                            self.flags = 0;
                            self.content_length = ULLONG_MAX;

                            if !is_alpha(ch) {
                                self.errno = error::HttpErrno::InvalidMethod;
                                return index;
                            }

                            self.method = http_method::HttpMethod::Delete;
                            self.index = 1;
                            match ch {
                                b'C' => self.method = http_method::HttpMethod::Connect, // or Copy, Checkout
                                b'D' => self.method = http_method::HttpMethod::Delete,
                                b'G' => self.method = http_method::HttpMethod::Get,
                                b'H' => self.method = http_method::HttpMethod::Head,
                                b'L' => self.method = http_method::HttpMethod::Lock,
                                b'M' => self.method = http_method::HttpMethod::MKCol, // or Move, MKActivity, Merge, MSearch, MKCalendar
                                b'N' => self.method = http_method::HttpMethod::Notify,
                                b'O' => self.method = http_method::HttpMethod::Options,
                                b'P' => self.method = http_method::HttpMethod::Post, // or PropFind|PropPatch|Put|Patch|Purge
                                b'R' => self.method = http_method::HttpMethod::Report,
                                b'S' => self.method = http_method::HttpMethod::Subscribe, // or Search
                                b'T' => self.method = http_method::HttpMethod::Trace,
                                b'U' => self.method = http_method::HttpMethod::Unlock, // or Unsubscribe
                                _ => {
                                    self.errno = error::HttpErrno::InvalidMethod;
                                    return index;
                                },
                            }
                            self.state = state::State::ReqMethod;

                            assert_ok!(self);
                            callback!(self, cb.on_message_begin(self), 
                                      error::HttpErrno::CBMessageBegin);
                            if self.errno != error::HttpErrno::Ok {
                                return index+1;
                            }
                        }
                    },
                    state::State::ReqMethod => {
                        if index == len {
                            self.errno = error::HttpErrno::InvalidMethod;
                            return index;
                        }

                        let matcher_string = self.method.to_string();
                        let matcher = matcher_string.as_slice();
                        if ch == b' ' && self.index == matcher.len() {
                            self.state = state::State::ReqSpacesBeforeUrl;
                        } else if ch == (matcher.char_at(self.index) as u8) {
                            ;
                        } else if self.method == http_method::HttpMethod::Connect {
                            if self.index == 1 && ch == b'H' {
                                self.method = http_method::HttpMethod::Checkout;
                            } else if self.index == 2 && ch == b'P' {
                                self.method = http_method::HttpMethod::Copy;
                            } else {
                                self.errno = error::HttpErrno::InvalidMethod;
                                return index;
                            }
                        } else if self.method == http_method::HttpMethod::MKCol {
                            if self.index == 1 && ch == b'O' {
                                self.method = http_method::HttpMethod::Move;
                            } else if self.index == 1 && ch == b'E' {
                                self.method = http_method::HttpMethod::Merge;
                            } else if self.index == 1 && ch == b'-' {
                                self.method = http_method::HttpMethod::MSearch;
                            } else if self.index == 2 && ch == b'A' {
                                self.method = http_method::HttpMethod::MKActivity;
                            } else if self.index == 3 && ch == b'A' {
                                self.method = http_method::HttpMethod::MKCalendar;
                            } else {
                                self.errno = error::HttpErrno::InvalidMethod;
                                return index;
                            }
                        } else if self.method == http_method::HttpMethod::Subscribe {
                            if self.index == 1 && ch == b'E' {
                                self.method = http_method::HttpMethod::Search;
                            } else {
                                self.errno == error::HttpErrno::InvalidMethod;
                                return index;
                            }
                        } else if self.index == 1 && self.method == http_method::HttpMethod::Post {
                           if ch == b'R' {
                               self.method = http_method::HttpMethod::PropFind; // or PropPatch
                           } else if ch == b'U' {
                               self.method = http_method::HttpMethod::Put; // or Purge
                           } else if ch == b'A' {
                               self.method = http_method::HttpMethod::Patch;
                           } else {
                               self.errno = error::HttpErrno::InvalidMethod;
                               return index;
                           }
                        } else if self.index == 2 {
                            if self.method == http_method::HttpMethod::Put {
                                if ch == b'R' {
                                    self.method = http_method::HttpMethod::Purge;
                                } else {
                                    self.errno = error::HttpErrno::InvalidMethod;
                                    return index;
                                }
                            } else if self.method == http_method::HttpMethod::Unlock {
                                if ch == b'S' {
                                    self.method = http_method::HttpMethod::Unsubscribe;
                                } else {
                                    self.errno = error::HttpErrno::InvalidMethod;
                                    return index;
                                }
                            } else {
                                self.errno = error::HttpErrno::InvalidMethod;
                                return index;
                            }
                        } else if self.index == 4 && self.method == http_method::HttpMethod::PropFind && ch == b'P' {
                            self.method = http_method::HttpMethod::PropPatch;
                        } else {
                            self.errno = error::HttpErrno::InvalidMethod;
                            return index;
                        }

                        self.index += 1;
                    },
                    state::State::ReqSpacesBeforeUrl => {
                        if ch != b' ' {
                            mark!(url_mark, index);
                            if self.method == http_method::HttpMethod::Connect {
                                self.state = state::State::ReqServerStart;
                            }

                            self.state = HttpParser::parse_url_char(self.state, ch);
                            if self.state == state::State::Dead {
                                self.errno = error::HttpErrno::InvalidUrl;
                                return index;
                            }
                        }
                    },
                    state::State::ReqSchema |
                    state::State::ReqSchemaSlash |
                    state::State::ReqSchemaSlashSlash |
                    state::State::ReqServerStart => {
                        match ch {
                            // No whitespace allowed here
                            b' ' | CR | LF => {
                                self.errno = error::HttpErrno::InvalidUrl;
                                return index;
                            },
                            _ => {
                                self.state = HttpParser::parse_url_char(self.state, ch);
                                if self.state == state::State::Dead {
                                    self.errno = error::HttpErrno::InvalidUrl;
                                    return index;
                                }
                            }
                        }
                    },
                    state::State::ReqServer |
                    state::State::ReqServerWithAt |
                    state::State::ReqPath |
                    state::State::ReqQueryStringStart |
                    state::State::ReqQueryString |
                    state::State::ReqFragmentStart |
                    state::State::ReqFragment => {
                        match ch {
                            b' ' => {
                                self.state = state::State::ReqHttpStart;
                                assert_ok!(self);
                                callback_data!(self, url_mark,
                                    cb.on_url(self, data.slice(url_mark.unwrap() as uint, index as uint)),
                                    error::HttpErrno::CBUrl, index+1);
                            },
                            CR | LF => {
                                self.http_major = 0;
                                self.http_minor = 9;
                                self.state = if ch == CR {
                                    state::State::ReqLineAlmostDone 
                                } else {
                                    state::State::HeaderFieldStart
                                };
                                assert_ok!(self);
                                callback_data!(self, url_mark,
                                    cb.on_url(self, data.slice(url_mark.unwrap() as uint, index as uint)),
                                    error::HttpErrno::CBUrl, index+1);
                            },
                            _ => {
                                self.state = HttpParser::parse_url_char(self.state, ch);
                                if self.state == state::State::Dead {
                                    self.errno = error::HttpErrno::InvalidUrl;
                                    return index;
                                }
                            }
                        }
                    },
                    state::State::ReqHttpStart => {
                        match ch {
                            b'H' => self.state = state::State::ReqHttpH,
                            b' ' => (),
                            _    => {
                                self.errno = error::HttpErrno::InvalidConstant;
                                return index;
                            }
                        }
                    },
                    state::State::ReqHttpH => {
                        strict_check!(self, ch != b'T', index);
                        self.state = state::State::ReqHttpHT;
                    },
                    state::State::ReqHttpHT => {
                        strict_check!(self, ch != b'T', index);
                        self.state = state::State::ReqHttpHTT;
                    },
                    state::State::ReqHttpHTT => {
                        strict_check!(self, ch != b'P', index);
                        self.state = state::State::ReqHttpHTTP;
                    },
                    state::State::ReqHttpHTTP => {
                        strict_check!(self, ch != b'/', index);
                        self.state = state::State::ReqFirstHttpMajor;
                    },
                    // first digit of major HTTP version
                    state::State::ReqFirstHttpMajor => {
                        if ch < b'1' || ch > b'9' {
                            self.errno = error::HttpErrno::InvalidVersion;
                            return index;
                        }

                        self.http_major = ch - b'0';
                        self.state = state::State::ReqHttpMajor;
                    },
                    // major HTTP version or dot
                    state::State::ReqHttpMajor => {
                        if ch == b'.' {
                            self.state = state::State::ReqFirstHttpMinor;
                        } else {
                            if !is_num(ch) {
                                self.errno = error::HttpErrno::InvalidVersion;
                                return index;
                            }

                            self.http_major *= 10;
                            self.http_major += ch - b'0';

                            if self.http_major > 99 {
                                self.errno = error::HttpErrno::InvalidVersion;
                                return index;
                            }
                        }
                    },
                    // first digit of minor HTTP version
                    state::State::ReqFirstHttpMinor => {
                        if !is_num(ch) {
                            self.errno = error::HttpErrno::InvalidVersion;
                            return index;
                        }

                        self.http_minor = ch - b'0';
                        self.state = state::State::ReqHttpMinor;
                    },
                    // minor HTTP version or end of request line
                    state::State::ReqHttpMinor => {
                        if ch == CR {
                            self.state = state::State::ReqLineAlmostDone;
                        } else if ch == LF {
                            self.state = state::State::HeaderFieldStart;
                        } else if !is_num(ch) {
                            // XXX allow spaces after digit?
                            self.errno = error::HttpErrno::InvalidVersion;
                            return index;
                        } else {
                            self.http_minor *= 10;
                            self.http_minor += ch - b'0';

                            if self.http_minor > 99 {
                                self.errno = error::HttpErrno::InvalidVersion;
                                return index;
                            }
                        }
                    },
                    // end of request line
                    state::State::ReqLineAlmostDone => {
                        if ch != LF {
                            self.errno = error::HttpErrno::LFExpected;
                            return index;
                        }

                        self.state = state::State::HeaderFieldStart;
                    },
                    state::State::HeaderFieldStart => {
                        if ch == CR {
                            self.state = state::State::HeadersAlmostDone;
                        } else if ch == LF {
                            // they might be just sending \n instead of \r\n,
                            // so this would be the second \n to denote
                            // the end of headers
                            self.state = state::State::HeadersAlmostDone;
                            retry = true;
                        } else {
                            let c : Option<u8> = token(ch);

                            if c.is_none() {
                                self.errno = error::HttpErrno::InvalidHeaderToken;
                                return index;
                            }

                            mark!(header_field_mark, index);
                            
                            self.index = 0;
                            self.state = state::State::HeaderField;

                            match c.unwrap() {
                                b'c' => self.header_state = state::HeaderState::C,
                                b'p' => self.header_state = state::HeaderState::MatchingProxyConnection,
                                b't' => self.header_state = state::HeaderState::MatchingTransferEncoding,
                                b'u' => self.header_state = state::HeaderState::MatchingUpgrade,
                                _    => self.header_state = state::HeaderState::General,
                            }
                        }
                    },
                    state::State::HeaderField => {
                        let c_opt : Option<u8> = token(ch);
                        if c_opt.is_some() {
                            let c : u8 = c_opt.unwrap();
                            match self.header_state {
                                state::HeaderState::General => (),
                                state::HeaderState::C => {
                                    self.index += 1;
                                    self.header_state = if c == b'o'{ 
                                        state::HeaderState::CO 
                                    } else {
                                        state::HeaderState::General
                                    };
                                },
                                state::HeaderState::CO => {
                                    self.index += 1;
                                    self.header_state = if c == b'n' {
                                        state::HeaderState::CON
                                    } else {
                                        state::HeaderState::General
                                    };
                                },
                                state::HeaderState::CON => {
                                    self.index += 1;
                                    match c {
                                        b'n' => self.header_state = state::HeaderState::MatchingConnection,
                                        b't' => self.header_state = state::HeaderState::MatchingContentLength,
                                        _    => self.header_state = state::HeaderState::General,
                                    }
                                },
                                // connection
                                state::HeaderState::MatchingConnection => {
                                    self.index += 1;
                                    if self.index >= CONNECTION.len() ||
                                        c != (CONNECTION.char_at(self.index) as u8) {
                                        self.header_state = state::HeaderState::General;
                                    } else if self.index == CONNECTION.len()-1 {
                                        self.header_state = state::HeaderState::Connection;
                                    }
                                },
                                // proxy-connection
                                state::HeaderState::MatchingProxyConnection => {
                                    self.index += 1;
                                    if self.index >= PROXY_CONNECTION.len() ||
                                        c != (PROXY_CONNECTION.char_at(self.index) as u8) {
                                        self.header_state = state::HeaderState::General;
                                    } else if self.index == PROXY_CONNECTION.len()-1 {
                                        self.header_state = state::HeaderState::Connection;
                                    }
                                },
                                // content-length
                                state::HeaderState::MatchingContentLength => {
                                    self.index += 1;
                                    if self.index >= CONTENT_LENGTH.len() ||
                                        c != (CONTENT_LENGTH.char_at(self.index) as u8) {
                                        self.header_state = state::HeaderState::General;
                                    } else if self.index == CONTENT_LENGTH.len()-1 {
                                        self.header_state = state::HeaderState::ContentLength;
                                    }
                                },
                                // transfer-encoding
                                state::HeaderState::MatchingTransferEncoding => {
                                    self.index += 1;
                                    if self.index >= TRANSFER_ENCODING.len() ||
                                        c != (TRANSFER_ENCODING.char_at(self.index) as u8) {
                                        self.header_state = state::HeaderState::General;
                                    } else if self.index == TRANSFER_ENCODING.len()-1 {
                                        self.header_state = state::HeaderState::TransferEncoding;
                                    }
                                },
                                // upgrade
                                state::HeaderState::MatchingUpgrade => {
                                    self.index += 1;
                                    if self.index >= UPGRADE.len() ||
                                        c != (UPGRADE.char_at(self.index) as u8) {
                                        self.header_state = state::HeaderState::General;
                                    } else if self.index == UPGRADE.len()-1 {
                                        self.header_state = state::HeaderState::Upgrade;
                                    }
                                },
                                state::HeaderState::Connection |
                                state::HeaderState::ContentLength |
                                state::HeaderState::TransferEncoding |
                                state::HeaderState::Upgrade => {
                                    if ch != b' ' {
                                        self.header_state = state::HeaderState::General;
                                    }
                                },
                                _ => {
                                    assert!(false, "Unknown header_state");
                                }
                            }
                        } else if ch == b':' {
                            self.state = state::State::HeaderValueDiscardWs;
                            assert_ok!(self);
                            callback_data!(self, header_field_mark,
                                cb.on_header_field(self, data.slice(header_field_mark.unwrap() as uint, index as uint)),
                                error::HttpErrno::CBHeaderField, index+1);
                        } else {
                            self.errno = error::HttpErrno::InvalidHeaderToken;
                            return index;
                        }
                    },
                    state::State::HeaderValueDiscardWs if ch == b' ' || ch == b'\t' ||
                        ch == CR || ch == LF => {
                        if ch == b' ' || ch == b'\t' {
                            ;
                        } else if ch == CR {
                            self.state = state::State::HeaderValueDiscardWsAlmostDone;
                        } else if ch == LF {
                            self.state = state::State::HeaderValueDiscardLws;
                        }
                    },
                    state::State::HeaderValueDiscardWs |
                    state::State::HeaderValueStart => {
                        mark!(header_value_mark, index);

                        self.state = state::State::HeaderValue;
                        self.index = 0;
                        
                        let c : u8 = lower(ch);

                        match self.header_state {
                            state::HeaderState::Upgrade => {
                                self.flags |= flags::flags::UPGRADE;
                                self.header_state = state::HeaderState::General;
                            },
                            state::HeaderState::TransferEncoding => {
                                // looking for 'Transfer-Encoding: chunked
                                if c == b'c' {
                                    self.header_state = state::HeaderState::MatchingTransferEncodingChunked;
                                } else {
                                    self.header_state = state::HeaderState::General;
                                }
                            },
                            state::HeaderState::ContentLength => {
                                if !is_num(ch) {
                                    self.errno = error::HttpErrno::InvalidContentLength;
                                    return index;
                                }

                                self.content_length = (ch - b'0') as u64;
                            },
                            state::HeaderState::Connection => {
                                // looking for 'Connection: keep-alive
                                if c == b'k' {
                                    self.header_state = state::HeaderState::MatchingConnectionKeepAlive;
                                // looking for 'Connection: close
                                } else if c == b'c' {
                                    self.header_state = state::HeaderState::MatchingConnectionClose;
                                } else {
                                    self.header_state = state::HeaderState::General;
                                }
                            },
                            _ => self.header_state = state::HeaderState::General,
                        }
                    },
                    state::State::HeaderValue => {
                        if ch == CR {
                            self.state = state::State::HeaderAlmostDone;
                            assert_ok!(self);
                            callback_data!(self, header_value_mark,
                                cb.on_header_value(self, data.slice(header_value_mark.unwrap() as uint, index as uint)),
                                error::HttpErrno::CBHeaderValue, index+1);
                        } else if ch == LF {
                            self.state = state::State::HeaderAlmostDone;
                            assert_ok!(self);
                            callback_data!(self, header_value_mark,
                                cb.on_header_value(self, data.slice(header_value_mark.unwrap() as uint, index as uint)),
                                error::HttpErrno::CBHeaderValue, index);
                            retry = true;
                        } else {
                            let c : u8 = lower(ch);

                            match self.header_state {
                                state::HeaderState::General => (),
                                state::HeaderState::Connection | state::HeaderState::TransferEncoding => {
                                    assert!(false, "Shouldn't get here.");
                                },
                                state::HeaderState::ContentLength => {
                                    if ch != b' ' {
                                        if !is_num(ch) {
                                            self.errno = error::HttpErrno::InvalidContentLength;
                                            return index;
                                        }

                                        let mut t : u64 = self.content_length;
                                        t *= 10;
                                        t += (ch - b'0') as u64;

                                        // Overflow? Test against a conservative
                                        // limit for simplicity
                                        if (ULLONG_MAX - 10) / 10 < self.content_length {
                                            self.errno = error::HttpErrno::InvalidContentLength;
                                            return index;
                                        }

                                        self.content_length = t;
                                    }
                                },
                                // Transfer-Encoding: chunked
                                state::HeaderState::MatchingTransferEncodingChunked => {
                                    self.index += 1;
                                    if self.index >= CHUNKED.len() ||
                                        c != (CHUNKED.char_at(self.index) as u8) {
                                            self.header_state = state::HeaderState::General;
                                    } else if self.index == CHUNKED.len()-1 {
                                        self.header_state = state::HeaderState::TransferEncodingChunked;
                                    }
                                },
                                // looking for 'Connection: keep-alive
                                state::HeaderState::MatchingConnectionKeepAlive => {
                                    self.index += 1;
                                    if self.index >= KEEP_ALIVE.len() ||
                                        c != (KEEP_ALIVE.char_at(self.index) as u8) {
                                            self.header_state = state::HeaderState::General;
                                    } else if self.index == KEEP_ALIVE.len()-1 {
                                        self.header_state = state::HeaderState::ConnectionKeepAlive;
                                    }
                                }
                                // looking for 'Connection: close
                                state::HeaderState::MatchingConnectionClose => {
                                    self.index += 1;
                                    if self.index >= CLOSE.len() ||
                                        c != (CLOSE.char_at(self.index) as u8) {
                                            self.header_state = state::HeaderState::General;
                                    } else if self.index == CLOSE.len()-1 {
                                        self.header_state = state::HeaderState::ConnectionClose;
                                    }
                                },
                                state::HeaderState::TransferEncodingChunked |
                                state::HeaderState::ConnectionKeepAlive |
                                state::HeaderState::ConnectionClose => {
                                    if ch != b' ' {
                                        self.header_state = state::HeaderState::General;
                                    }
                                },
                                _ => {
                                    self.state = state::State::HeaderValue;
                                    self.header_state = state::HeaderState::General;
                                }
                            }
                        }
                    },
                    state::State::HeaderAlmostDone => {
                        strict_check!(self, ch != LF, index);

                        self.state = state::State::HeaderValueLws;
                    },
                    state::State::HeaderValueLws => {
                        if ch == b' ' || ch == b'\t' {
                            self.state = state::State::HeaderValueStart;
                            retry = true;
                        } else {
                            // finished the header
                            match self.header_state {
                                state::HeaderState::ConnectionKeepAlive => {
                                    self.flags |= flags::flags::CONNECTION_KEEP_ALIVE;
                                },
                                state::HeaderState::ConnectionClose => {
                                    self.flags |= flags::flags::CONNECTION_CLOSE;
                                },
                                state::HeaderState::TransferEncodingChunked => {
                                    self.flags |= flags::flags::CHUNKED;
                                },
                                _ => (),
                            }

                            self.state = state::State::HeaderFieldStart;
                            retry = true;
                        }
                    },
                    state::State::HeaderValueDiscardWsAlmostDone => {
                        strict_check!(self, ch != LF, index);
                        self.state = state::State::HeaderValueDiscardLws;
                    },
                    state::State::HeaderValueDiscardLws => {
                        if ch == b' ' || ch == b'\t' {
                            self.state = state::State::HeaderValueDiscardWs;
                        } else {
                            // header value was empty
                            mark!(header_value_mark, index);
                            self.state = state::State::HeaderFieldStart;
                            assert_ok!(self);
                            callback_data!(self, header_value_mark,
                                cb.on_header_value(self, data.slice(header_value_mark.unwrap() as uint, index as uint)),
                                error::HttpErrno::CBHeaderValue, index);
                            retry = true;
                        }
                    },
                    state::State::HeadersAlmostDone => {
                        strict_check!(self, ch != LF, index);

                        if (self.flags & flags::flags::TRAILING) > 0 {
                            // End of a chunked request
                            self.state = new_message!(self);
                            assert_ok!(self);
                            callback!(self, cb.on_message_complete(self), 
                                      error::HttpErrno::CBMessageComplete);
                            if self.errno != error::HttpErrno::Ok {
                                return index+1;
                            }
                        } else {
                            self.state = state::State::HeadersDone;

                            // Set this here so that on_headers_complete()
                            // callbacks can see it
                            self.upgrade = (self.flags & flags::flags::UPGRADE != 0) ||
                                self.method == http_method::HttpMethod::Connect;

                            // Here we call the headers_complete callback. This is somewhat
                            // different than other callbacks because if the user returns 1, we
                            // will interpret that as saying that this message has no body. This
                            // is needed for the annoying case of recieving a response to a HEAD
                            // request.
                            // 
                            // We'd like to use CALLBACK_NOTIFY_NOADVANCE() here but we cannot,
                            // so
                            // we have to simulate it by handling a change in errno below.
                            //
                            // TODO can we handle this in our case?
                            match cb.on_headers_complete(self) {
                                Ok(0) => (),
                                Ok(1) => self.flags |= flags::flags::SKIPBODY,
                                _     => {
                                    self.errno = error::HttpErrno::CBHeadersComplete;
                                    return index; // Error
                                },
                            }

                            if self.errno != error::HttpErrno::Ok {
                                return index;
                            }
                            retry = true;
                        }
                    },
                    state::State::HeadersDone => {
                        strict_check!(self, ch != LF, index);
                        self.nread = 0;

                        // Exit, The rest of the connect is in a different protocal
                        if self.upgrade {
                            self.state = new_message!(self);
                            assert_ok!(self);
                            callback!(self, cb.on_message_complete(self), 
                                      error::HttpErrno::CBMessageComplete);
                            if self.errno != error::HttpErrno::Ok {
                                return index+1;
                            }
                            return index+1;
                        }

                        if (self.flags & flags::flags::SKIPBODY) != 0 {
                            self.state = new_message!(self);
                            assert_ok!(self);
                            callback!(self, cb.on_message_complete(self), 
                                      error::HttpErrno::CBMessageComplete);
                            if self.errno != error::HttpErrno::Ok {
                                return index+1;
                            }
                        } else if (self.flags & flags::flags::CHUNKED) != 0 {
                            // chunked encoding - ignore Content-Length header
                            self.state = state::State::ChunkSizeStart;
                        } else {
                            if self.content_length == 0 {
                                // Content-Length header given but zero: Content-Length: 0\r\n
                                self.state = new_message!(self);
                                assert_ok!(self);
                                callback!(self, cb.on_message_complete(self), 
                                          error::HttpErrno::CBMessageComplete);
                                if self.errno != error::HttpErrno::Ok {
                                    return index+1;
                                }
                            } else if self.content_length != ULLONG_MAX {
                                // Content-Length header given and non-zero
                                self.state = state::State::BodyIdentity;
                            } else {
                                if self.tp == HttpParserType::HttpRequest ||
                                    !self.http_message_needs_eof() {
                                    // Assume content-length 0 - read the next
                                    self.state = new_message!(self);
                                    assert_ok!(self);
                                    callback!(self, cb.on_message_complete(self), 
                                              error::HttpErrno::CBMessageComplete);
                                    if self.errno != error::HttpErrno::Ok {
                                        return index+1;
                                    }
                                } else {
                                    // Read body until EOF
                                    self.state = state::State::BodyIdentityEof;
                                }
                            }
                        }
                    },
                    state::State::BodyIdentity => {
                        let to_read : u64 = cmp::min(self.content_length,
                                                    (len - index) as u64);
                        assert!(self.content_length != 0 &&
                                self.content_length != ULLONG_MAX);

                        // The difference between advancing content_length and p is because
                        // the latter will automaticaly advance on the next loop iteration.
                        // Further, if content_length ends up at 0, we want to see the last
                        // byte again for our message complete callback.
                        mark!(body_mark, index);
                        self.content_length -= to_read;

                        index += to_read - 1;

                        if self.content_length == 0 {
                            self.state = state::State::MessageDone;

                            // Mimic CALLBACK_DATA_NOADVANCE() but with one extra byte.
                            //
                            // The alternative to doing this is to wait for the next byte to
                            // trigger the data callback, just as in every other case. The
                            // problem with this is that this makes it difficult for the test
                            // harness to distinguish between complete-on-EOF and
                            // complete-on-length. It's not clear that this distinction is
                            // important for applications, but let's keep it for now.
                            assert_ok!(self);
                            callback_data!(self, body_mark,
                                cb.on_body(self, data.slice(body_mark.unwrap() as uint, (index + 1) as uint)),
                                error::HttpErrno::CBBody, index);
                            retry = true;
                        }
                    },
                    // read until EOF
                    state::State::BodyIdentityEof => {
                        mark!(body_mark, index);
                        index = len - 1;
                    },
                    state::State::MessageDone => {
                        self.state = new_message!(self);
                        assert_ok!(self);
                        callback!(self, cb.on_message_complete(self), 
                                  error::HttpErrno::CBMessageComplete);
                        if self.errno != error::HttpErrno::Ok {
                            return index+1;
                        }
                    },
                    state::State::ChunkSizeStart => {
                        assert!(self.nread == 1);
                        assert!(self.flags & flags::flags::CHUNKED != 0);

                        let unhex_val : i8 = UNHEX[ch as uint];
                        if unhex_val == -1 {
                            self.errno = error::HttpErrno::InvalidChunkSize;
                            return index;
                        }

                        self.content_length = unhex_val as u64;
                        self.state = state::State::ChunkSize;
                    },
                    state::State::ChunkSize => {
                        assert!(self.flags & flags::flags::CHUNKED != 0);

                        if ch == CR {
                            self.state = state::State::ChunkSizeAlmostDone;
                        } else {
                            let unhex_val : i8 = UNHEX[ch as uint];
                            if unhex_val == -1 {
                                if ch == b';' || ch == b' ' {
                                    self.state = state::State::ChunkParameters;
                                } else {
                                    self.errno = error::HttpErrno::InvalidChunkSize;
                                    return index;
                                }
                            } else {
                                let mut t : u64 = self.content_length;
                                t *= 16;
                                t += unhex_val as u64;

                                // Overflow? Test against a conservative limit for simplicity
                                if (ULLONG_MAX - 16)/16 < self.content_length {
                                    self.errno = error::HttpErrno::InvalidContentLength;
                                    return index;
                                }

                                self.content_length = t;
                            }
                        }
                    },
                    state::State::ChunkParameters => {
                        assert!(self.flags & flags::flags::CHUNKED != 0);
                        // just ignore this shit. TODO check for overflow
                        if ch == CR {
                            self.state = state::State::ChunkSizeAlmostDone;
                        }
                    },
                    state::State::ChunkSizeAlmostDone => {
                        assert!(self.flags & flags::flags::CHUNKED != 0);
                        strict_check!(self, ch != LF, index);

                        self.nread = 0;

                        if self.content_length == 0 {
                            self.flags |= flags::flags::TRAILING;
                            self.state = state::State::HeaderFieldStart;
                        } else {
                            self.state = state::State::ChunkData;
                        }
                    },
                    state::State::ChunkData => {
                        let to_read : u64 = cmp::min(self.content_length,
                                                         len - index);
                        assert!(self.flags & flags::flags::CHUNKED != 0);
                        assert!(self.content_length != 0 &&
                                self.content_length != ULLONG_MAX);

                        // See the explanation in s_body_identity for why the content
                        // length and data pointers are managed this way.
                        mark!(body_mark, index);
                        self.content_length -= to_read;
                        index += to_read - 1;

                        if self.content_length == 0 {
                            self.state = state::State::ChunkDataAlmostDone;
                        }
                    },
                    state::State::ChunkDataAlmostDone => {
                        assert!(self.flags & flags::flags::CHUNKED != 0);
                        assert!(self.content_length == 0);
                        strict_check!(self, ch != CR, index);
                        self.state = state::State::ChunkDataDone;

                        assert_ok!(self);
                        callback_data!(self, body_mark,
                            cb.on_body(self, data.slice(body_mark.unwrap() as uint, index as uint)),
                            error::HttpErrno::CBBody, index+1);
                    },
                    state::State::ChunkDataDone => {
                        assert!(self.flags & flags::flags::CHUNKED != 0);
                        strict_check!(self, ch != LF, index);
                        self.nread = 0;
                        self.state = state::State::ChunkSizeStart;
                    },
                    //_ => {
                    //    assert!(false, "unhandled state");
                    //    self.errno = error::HttpErrno::InvalidInternalState;
                    //    return index;
                    //},
                }

                if !retry {
                    break;
                }
            }
            index += 1;
        }

        // Run callbacks for any marks that we have leftover after we ran our of
        // bytes. There should be at most one of these set, so it's OK to invoke
        // them in series (unset marks will not result in callbacks).
        //
        // We use the NOADVANCE() variety of callbacks here because 'p' has already
        // overflowed 'data' and this allows us to correct for the off-by-one that
        // we'd otherwise have (since CALLBACK_DATA() is meant to be run with a 'p'
        // value that's in-bounds).
        assert!((if header_field_mark.is_some() { 1u8 } else { 0 }) +
                (if header_value_mark.is_some() { 1 } else { 0 }) +
                (if url_mark.is_some() { 1 } else { 0 }) +
                (if body_mark.is_some() { 1 } else { 0 }) +
                (if status_mark.is_some() { 1 } else { 0 }) <= 1);

        callback_data!(self, header_field_mark,
            cb.on_header_field(self, data.slice(header_field_mark.unwrap() as uint, index as uint)),
            error::HttpErrno::CBHeaderField, index);
        callback_data!(self, header_value_mark,
            cb.on_header_value(self, data.slice(header_value_mark.unwrap() as uint, index as uint)),
            error::HttpErrno::CBHeaderValue, index);
        callback_data!(self, url_mark,
            cb.on_url(self, data.slice(url_mark.unwrap() as uint, index as uint)),
            error::HttpErrno::CBUrl, index);
        callback_data!(self, body_mark,
            cb.on_body(self, data.slice(body_mark.unwrap() as uint, index as uint)),
            error::HttpErrno::CBBody, index);
        callback_data!(self, status_mark,
            cb.on_status(self, data.slice(status_mark.unwrap() as uint, index as uint)),
            error::HttpErrno::CBStatus, index);
        len
    }

    pub fn http_body_is_final(&self) -> bool {
        self.state == state::State::MessageDone
    }

    // Our URL parser
    fn parse_url_char(s : state::State, ch : u8) -> state::State {
        if ch == b' ' || ch == b'\r' || ch == b'\n' {
            return state::State::Dead;
        }

        if HTTP_PARSER_STRICT {
            if ch == b'\t' || ch == b'\x0C' {   // '\x0C' = '\f' 
                return state::State::Dead;
            }
        }

        match s {
            state::State::ReqSpacesBeforeUrl => {
                // Proxied requests are followed by scheme of an absolute URI (alpha).
                // All methods except CONNECT are followed by '/' or '*'.

                if ch == b'/' || ch == b'*' {
                    return state::State::ReqPath;
                }

                if is_alpha(ch) {
                    return state::State::ReqSchema;
                }
            },
            state::State::ReqSchema => {
                if is_alpha(ch) {
                    return s;
                }

                if ch == b':' {
                    return state::State::ReqSchemaSlash;
                }
            },
            state::State::ReqSchemaSlash => {
                if ch == b'/' {
                    return state::State::ReqSchemaSlashSlash;
                }
            },
            state::State::ReqSchemaSlashSlash => {
                if ch == b'/' {
                    return state::State::ReqServerStart;
                }
            },
            state::State::ReqServerWithAt if ch == b'@' => return state::State::Dead,
            state::State::ReqServerWithAt | state::State::ReqServerStart | state::State::ReqServer => {
                if ch == b'/' {
                    return state::State::ReqPath;
                }

                if ch == b'?' {
                    return state::State::ReqQueryStringStart;
                }

                if ch == b'@' {
                    return state::State::ReqServerWithAt;
                }

                if is_userinfo_char(ch) || ch == b'[' || ch == b']' {
                    return state::State::ReqServer;
                }
            },
            state::State::ReqPath => {
                if is_url_char(ch) {
                    return s;
                }

                match ch {
                    b'?' => return state::State::ReqQueryStringStart,
                    b'#' => return state::State::ReqFragmentStart,
                    _    => (),
                }
            },
            state::State::ReqQueryStringStart | state::State::ReqQueryString => {
                if is_url_char(ch) {
                    return state::State::ReqQueryString;
                }

                match ch {
                    b'?' => return state::State::ReqQueryString, // allow extra '?' in query string
                    b'#' => return state::State::ReqFragmentStart,
                    _    => (),
                }
            },
            state::State::ReqFragmentStart => {
                if is_url_char(ch) {
                    return state::State::ReqFragment;
                }

                match ch {
                    b'?' => return state::State::ReqFragment,
                    b'#' => return s,
                    _    => (),
                }
            },
            state::State::ReqFragment => {
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
        return state::State::Dead;
    }

    // Does the parser need to see an EOF to find the end of the message?
    fn http_message_needs_eof(&self) -> bool {
        if self.tp == HttpParserType::HttpRequest {
            return false
        }

        // See RFC 2616 section 4.4
        if self.status_code / 100 == 1 || // 1xx e.g. Continue
            self.status_code == 204 ||    // No Content
            self.status_code == 304 ||    // Not Modified
            (self.flags & flags::flags::SKIPBODY) != 0 {// response to a HEAD request
            return false
        }

        if (self.flags & flags::flags::CHUNKED != 0) ||
            self.content_length != ULLONG_MAX {
            return false
        }

        true
    }

    pub fn http_should_keep_alive(&self) -> bool {
        if self.http_major > 0 && self.http_minor > 0 {
            // HTTP/1.1
            if (self.flags & flags::flags::CONNECTION_CLOSE) != 0 {
                return false
            }
        } else {
            // HTTP/1.0 or earlier
            if (self.flags & flags::flags::CONNECTION_KEEP_ALIVE) == 0 {
                return false
            }
        }

        !self.http_message_needs_eof()
    }

}

// for tests only. Should be deleted after tests are done
fn main() {
    test_responses();
}

fn test_responses() {
    // RESPONSES
    let responses: [Message, ..1] = [
        Message {
            name: String::from_str("non-ASCII in status line"),
            tp: HttpParserType::HttpResponse,
            raw: String::from_str(
                "HTTP/1.1 500 Oriëntatieprobleem\r\n\
                Date: Fri, 5 Nov 2010 23:07:12 GMT+2\r\n\
                Content-Length: 0\r\n\
                Connection: close\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_major: 1,
            http_minor: 1,
            status_code: 500,
            response_status: String::from_str("Oriëntatieprobleem"),
            num_headers: 3,
            headers: vec![
                [ String::from_str("Date"), String::from_str("Fri, 5 Nov 2010 23:07:12 GMT+2") ],
                [ String::from_str("Content-Length"), String::from_str("0") ],
                [ String::from_str("Connection"), String::from_str("close") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
    ];
    // End of RESPONSES
    for m in responses.iter() {
        test_message(m);
    }
}

fn test_message(message: &Message) {
    let raw = &message.raw;
    let raw_len = raw.len();
    for i in range(0, raw_len) {
        println!("at {}", i);
        let mut hp = HttpParser::new(message.tp);
        let mut cb = CallbackRegular{..Default::default()};
        cb.messages.push(Message{..Default::default()});
        let mut read: u64 = 0;

        if i > 0 {
            read = hp.execute(&mut cb, raw.slice(0, i).as_bytes());

            if !message.upgrade.is_empty() && hp.upgrade {
                cb.messages[cb.num_messages - 1].upgrade = raw.slice_from(read as uint).to_string();
                assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", message.name);
                assert_eq_message(&cb.messages[0], message);
                continue;
            }

            if read != (i as u64) {
                print_error(hp.errno, raw.as_slice(), read);
                panic!();
            }
        }

        read = hp.execute(&mut cb, raw.slice_from(i).as_bytes());

        if !(message.upgrade.is_empty()) && hp.upgrade {
            cb.messages[cb.num_messages - 1].upgrade = raw.slice_from(i+(read as uint)).to_string();
            assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", message.name);
            assert_eq_message(&cb.messages[0], message);
            continue;
        }

        if read != ((raw_len - i) as u64) {
            print_error(hp.errno, raw.as_slice(), (i + read as uint) as u64);
            panic!();
        }

        cb.currently_parsing_eof = true;
        read = hp.execute(&mut cb, &[]);

        if (read != 0) {
            print_error(hp.errno, raw.as_slice(), read);
            panic!();
        }

        assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", message.name);
        assert_eq_message(&cb.messages[0], message);
    }
}

#[deriving(PartialEq, Eq )]
pub enum LastHeaderType {
    None,
    Field,
    Value,
}

pub struct Message {
    pub name: String,
    pub raw: String,
    pub tp: HttpParserType,
    pub method: HttpMethod,
    pub status_code: u16,
    pub response_status: String,
    pub request_path: String,
    pub request_url: String,
    pub fragment: String,
    pub query_string: String,
    pub body: String,
    pub body_size: uint, // maybe not necessary
    pub host: String,
    pub userinfo: String,
    pub port: u16,
    pub num_headers: int, // might be able to delete this
    pub last_header_element: LastHeaderType,
    pub headers: Vec<[String,..2]>,
    pub should_keep_alive: bool,
    
    pub upgrade: String,

    pub http_major: u8,
    pub http_minor: u8,

    pub message_begin_cb_called: bool,
    pub headers_complete_cb_called: bool,
    pub message_complete_cb_called: bool,
    pub message_complete_on_eof: bool,
    pub body_is_final: bool,
}

impl Default for Message {
    fn default() -> Message {
        Message {
            name: String::new() ,
            raw: String::new(),
            tp: HttpParserType::HttpBoth,
            method: HttpMethod::Delete,
            status_code: 0,
            response_status: String::new(),
            request_path: String::new(),
            request_url: String::new(),
            fragment: String::new(),
            query_string: String::new(),
            body: String::new(),
            body_size: 0,
            host: String::new(),
            userinfo: String::new(),
            port: 0,
            num_headers: 0,
            last_header_element: LastHeaderType::None,
            headers: vec![],
            should_keep_alive: false,

            upgrade: String::new(),
            
            http_major: 0,
            http_minor: 0,

            message_begin_cb_called: false,
            headers_complete_cb_called: false,
            message_complete_cb_called: false,
            message_complete_on_eof: false,
            body_is_final: false,
        }
    }
}

pub struct CallbackEmpty;

impl HttpParserCallback for CallbackEmpty {}

pub struct CallbackRegular {
    pub num_messages: uint, // maybe not necessary
    pub messages: Vec<Message>,
    pub currently_parsing_eof: bool,
}

impl Default for CallbackRegular {
    fn default() -> CallbackRegular {
        CallbackRegular {
            num_messages: 0,
            messages: Vec::new(),
            currently_parsing_eof: false,
        }
    }
}

impl HttpParserCallback for CallbackRegular {
    fn on_message_begin(&mut self, parser : &HttpParser) -> Result<i8, &str> {
        self.messages[self.num_messages].message_begin_cb_called = true;
        Ok(0)
    }

    fn on_url(&mut self, parser : &HttpParser, data : &[u8]) -> Result<i8, &str> {
        match str::from_utf8(data) {
            Result::Ok(data_str) => {
                self.messages[self.num_messages].request_url.push_str(
                    data_str);
            },
            _ => panic!("on_url: data is not in utf8 encoding"),
        }
        Ok(0)
    }

    fn on_status(&mut self, parser : &HttpParser, data : &[u8]) -> Result<i8, &str> {
        match str::from_utf8(data) {
            Result::Ok(data_str) => {
                self.messages[self.num_messages].response_status.push_str(
                    data_str);
            },
            _ => panic!("on_status: data is not in utf8 encoding"),
        }
        Ok(0)
    }

    fn on_header_field(&mut self, parser : &HttpParser, data : &[u8]) -> Result<i8, &str> {
        let m : &mut Message = &mut self.messages[self.num_messages];

        if m.last_header_element != LastHeaderType::Field {
            m.num_headers += 1;
            m.headers.push([String::new(), String::new()]);
        }
        
        match str::from_utf8(data) {
            Result::Ok(data_str) => {
                let i = m.headers.len()-1;
                m.headers[i][0].push_str(data_str);
            },
            _ => panic!("on_header_field: data is not in utf8 encoding"),
        }

        m.last_header_element = LastHeaderType::Field;

        Ok(0)
    }

    fn on_header_value(&mut self, parser : &HttpParser, data : &[u8]) -> Result<i8, &str> {
        let m : &mut Message = &mut self.messages[self.num_messages];

        match str::from_utf8(data) {
            Result::Ok(data_str) => {
                let i = m.headers.len()-1;
                m.headers[i][1].push_str(data_str);
            },
            _ => panic!("on_header_value: data is not in utf8 encoding"),
        }

        m.last_header_element = LastHeaderType::Value;

        Ok(0)
    }

    fn on_headers_complete(&mut self, parser : &HttpParser) -> Result<i8, &str> {
        let m : &mut Message = &mut self.messages[self.num_messages];
        m.method = parser.method;
        m.status_code = parser.status_code;
        m.http_major = parser.http_major;
        m.http_minor = parser.http_minor;
        m.headers_complete_cb_called = true;
        m.should_keep_alive = parser.http_should_keep_alive();
        Ok(0)
    }

    fn on_body(&mut self, parser : &HttpParser, data : &[u8]) -> Result<i8, &str> {
        let m : &mut Message = &mut self.messages[self.num_messages];

        match str::from_utf8(data) {
            Result::Ok(data_str) => {
                m.body.push_str(data_str);
            },
            _ => panic!("on_body: data is not in utf8 encoding"),
        }
        m.body_size += data.len();

        if m.body_is_final {
            panic!("\n\n ** Error http_body_is_final() should return 1 \
                    on last on_body callback call \
                    but it doesn't! **\n\n");
        }

        m.body_is_final = parser.http_body_is_final();
        Ok(0)
    }

    fn on_message_complete(&mut self, parser : &HttpParser) -> Result<i8, &str> {
        {
            let m : &mut Message = &mut self.messages[self.num_messages];

            if m.should_keep_alive != parser.http_should_keep_alive() {
                panic!("\n\n ** Error http_should_keep_alive() should have same \
                        value in both on_message_complete and on_headers_complet \
                        but it doesn't! **\n\n");
            }

            if m.body_size > 0 && parser.http_body_is_final()
                && !m.body_is_final {
                panic!("\n\n ** Error http_body_is_final() should return 1 \
                        on last on_body callback call \
                        but it doesn't! **\n\n");
            }

            m.message_complete_cb_called = true;
            m.message_complete_on_eof = self.currently_parsing_eof;
        }

        self.messages.push(Message{..Default::default()});
        self.num_messages += 1;

        Ok(0)
    }
}

pub fn print_error(errno: HttpErrno, raw: &str, error_location: u64) {
    println!("\n*** {} ***\n", errno.to_string());

    let len = raw.len();
    let mut this_line = false;
    let mut char_len: u64 = 0;
    let mut error_location_line = 0;
    let mut eof = true;
    for i in range(0, len) {
        if i == (error_location as uint) { this_line = true; }
        match raw.char_at(i) {
            '\r' => {
                char_len = 2;
                print!("\\r");
            },
            '\n' => {
                println!("\\n");

                if this_line {
                    eof = false;
                    break;
                }

                error_location_line = 0;
                continue;
            },
            _ => {
                char_len = 1;
                print!("{}", raw.char_at(i));
            },
        }
        if !this_line { error_location_line += char_len; }       
    }

    if eof {
        println!("[eof]");
    }

    for i in range(0, error_location_line as u64) {
        print!(" ");
    }
    println!("^\n\nerror location: {}", error_location);
}

pub fn assert_eq_message(actual: &Message, expected: &Message) {
    assert_eq!(actual.http_major, expected.http_major);
    assert_eq!(actual.http_minor, expected.http_minor);

    if expected.tp == HttpParserType::HttpRequest {
        assert!(actual.method == expected.method);
    } else {
        assert_eq!(actual.status_code, expected.status_code);
        assert_eq!(actual.response_status, expected.response_status);
    }

    assert_eq!(actual.should_keep_alive, expected.should_keep_alive);
    assert_eq!(actual.message_complete_on_eof, expected.message_complete_on_eof);

    assert!(actual.message_begin_cb_called);
    assert!(actual.headers_complete_cb_called);
    assert!(actual.message_complete_cb_called);

    assert_eq!(actual.request_url, expected.request_url);

    // Check URL components; we can't do this w/ CONNECT since it doesn't
    // send us a well-formed URL.
    // TODO add after implementing http_parser_parse_url()

    if expected.body_size > 0 {
        assert_eq!(actual.body_size, expected.body_size);
    } else {
        assert_eq!(actual.body, expected.body);
    }

    assert_eq!(actual.num_headers, expected.num_headers);

    for i in range(0, actual.num_headers) {
        assert_eq!(actual.headers[i as uint][0], expected.headers[i as uint][0]);
        assert_eq!(actual.headers[i as uint][1], expected.headers[i as uint][1]);
    }

    assert_eq!(actual.upgrade, expected.upgrade);
}
