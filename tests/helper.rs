extern crate http_parser;

use std::default::Default;
use std::str;

use self::http_parser::{HttpParser, HttpParserCallback, HttpParserType, HttpMethod, HttpErrno};

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
