extern crate http_parser;

use std::default::Default;
use std::str;

use self::http_parser::*;

#[derive(PartialEq, Eq )]
pub enum LastHeaderType {
    None,
    Field,
    Value,
}

pub struct Message {
    pub name: String,
    pub raw: String,
    pub tp: HttpParserType,
    pub strict: bool,
    pub method: Option<HttpMethod>,
    pub status_code: Option<u16>,
    pub response_status: Vec<u8>,
    pub request_path: String,
    pub request_url: Vec<u8>,
    pub fragment: String,
    pub query_string: String,
    pub body: String,
    pub body_size: usize, // maybe not necessary
    pub host: String,
    pub userinfo: String,
    pub port: u16,
    pub num_headers: i32, // might be able to delete this
    pub last_header_element: LastHeaderType,
    pub headers: Vec<[String; 2]>,
    pub should_keep_alive: bool,
    
    pub upgrade: Option<String>,

    pub http_version: HttpVersion,

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
            tp: HttpParserType::Both,
            strict: true,
            method: None,
            status_code: None,
            response_status: vec![],
            request_path: String::new(),
            request_url: vec![],
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

            upgrade: None,
            
            http_version: HttpVersion { major: 0, minor: 0 },

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
    pub num_messages: usize, // maybe not necessary
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
    fn on_message_begin(&mut self, _ : &mut HttpParser) -> CallbackResult {
        self.messages[self.num_messages].message_begin_cb_called = true;
        Ok(CallbackDecision::Nothing)
    }

    fn on_url(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
        for b in data {
            self.messages[self.num_messages].request_url.push(*b);
        }
        Ok(CallbackDecision::Nothing)
    }

    fn on_status(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
        for b in data {
            self.messages[self.num_messages].response_status.push(*b);
        }
        Ok(CallbackDecision::Nothing)
    }

    fn on_header_field(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
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

        Ok(CallbackDecision::Nothing)
    }

    fn on_header_value(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
        let m : &mut Message = &mut self.messages[self.num_messages];

        match str::from_utf8(data) {
            Result::Ok(data_str) => {
                let i = m.headers.len()-1;
                m.headers[i][1].push_str(data_str);
            },
            _ => panic!("on_header_value: data is not in utf8 encoding"),
        }

        m.last_header_element = LastHeaderType::Value;

        Ok(CallbackDecision::Nothing)
    }

    fn on_headers_complete(&mut self, parser : &mut HttpParser) -> CallbackResult {
        let m : &mut Message = &mut self.messages[self.num_messages];
        m.method = parser.method;
        m.status_code = parser.status_code;
        m.http_version = parser.http_version;
        m.headers_complete_cb_called = true;
        m.should_keep_alive = parser.http_should_keep_alive();
        Ok(CallbackDecision::Nothing)
    }

    fn on_body(&mut self, parser : &mut HttpParser, data : &[u8]) -> CallbackResult {
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
        Ok(CallbackDecision::Nothing)
    }

    fn on_message_complete(&mut self, parser : &mut HttpParser) -> CallbackResult {
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

        Ok(CallbackDecision::Nothing)
    }
}

pub struct CallbackDontCall;

impl HttpParserCallback for CallbackDontCall {
    fn on_message_begin(&mut self, _ : &mut HttpParser) -> CallbackResult {
        panic!("\n\n*** on_message_begin() called on paused parser ***\n\n");
    }

    #[allow(unused_variables)]
    fn on_url(&mut self, _ : &mut HttpParser, data : &[u8],) -> CallbackResult {
        panic!("\n\n*** on_url() called on paused parser ***\n\n");
    }

    #[allow(unused_variables)]
    fn on_status(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
        panic!("\n\n*** on_status() called on paused parser ***\n\n");
    }

    #[allow(unused_variables)]
    fn on_header_field(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
        panic!("\n\n*** on_header_field() called on paused parser ***\n\n");
    }

    #[allow(unused_variables)]
    fn on_header_value(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
        panic!("\n\n*** on_header_value() called on paused parser ***\n\n");
    }

    fn on_headers_complete(&mut self, _ : &mut HttpParser) -> CallbackResult {
        panic!("\n\n*** on_headers_complete() called on paused parser ***\n\n");
    }

    #[allow(unused_variables)]
    fn on_body(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
        panic!("\n\n*** on_body() called on paused parser ***\n\n");
    }

    fn on_message_complete(&mut self, _ : &mut HttpParser) -> CallbackResult {
        panic!("\n\n*** on_message_complete() called on paused parser ***\n\n");
    }
}

pub struct CallbackPause {
    pub num_messages: usize, // maybe not necessary
    pub messages: Vec<Message>,
    pub currently_parsing_eof: bool,

    pub paused: bool,
    dontcall: CallbackDontCall,
}

impl Default for CallbackPause {
    fn default() -> CallbackPause {
        CallbackPause {
            num_messages: 0,
            messages: Vec::new(),
            currently_parsing_eof: false,
            paused: false,
            dontcall: CallbackDontCall,
        }
    }
}

// TODO try to reuse code from CallbackRegular
impl HttpParserCallback for CallbackPause {
    fn on_message_begin(&mut self, parser : &mut HttpParser) -> CallbackResult {
        if self.paused {
            self.dontcall.on_message_begin(parser)
        } else {
            parser.pause(true);
            self.paused = true;
            self.messages[self.num_messages].message_begin_cb_called = true;
            Ok(CallbackDecision::Nothing)
        }
    }

    fn on_url(&mut self, parser : &mut HttpParser, data : &[u8],) -> CallbackResult {
        if self.paused {
            self.dontcall.on_url(parser, data)
        } else {
            parser.pause(true);
            self.paused = true;
            for b in data {
                self.messages[self.num_messages].request_url.push(*b);
            }
            Ok(CallbackDecision::Nothing)
        }
    }

    fn on_status(&mut self, parser : &mut HttpParser, data : &[u8]) -> CallbackResult {
        if self.paused {
            self.dontcall.on_status(parser, data)
        } else {
            parser.pause(true);
            self.paused = true;
            for b in data {
                self.messages[self.num_messages].response_status.push(*b);
            }
            Ok(CallbackDecision::Nothing)
        }
    }

    fn on_header_field(&mut self, parser : &mut HttpParser, data : &[u8]) -> CallbackResult {
        if self.paused {
            self.dontcall.on_header_field(parser, data)
        } else {
            parser.pause(true);
            self.paused = true;
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

            Ok(CallbackDecision::Nothing)
        }
    }

    fn on_header_value(&mut self, parser : &mut HttpParser, data : &[u8]) -> CallbackResult {
        if self.paused {
            self.dontcall.on_header_value(parser, data)
        } else {
            parser.pause(true);
            self.paused = true;
            let m : &mut Message = &mut self.messages[self.num_messages];

            match str::from_utf8(data) {
                Result::Ok(data_str) => {
                    let i = m.headers.len()-1;
                    m.headers[i][1].push_str(data_str);
                },
                _ => panic!("on_header_value: data is not in utf8 encoding"),
            }

            m.last_header_element = LastHeaderType::Value;

            Ok(CallbackDecision::Nothing)
        }
    }

    fn on_headers_complete(&mut self, parser : &mut HttpParser) -> CallbackResult {
        if self.paused {
            self.dontcall.on_headers_complete(parser)
        } else {
            parser.pause(true);
            self.paused = true;
            let m : &mut Message = &mut self.messages[self.num_messages];
            m.method = parser.method;
            m.status_code = parser.status_code;
            m.http_version = parser.http_version;
            m.headers_complete_cb_called = true;
            m.should_keep_alive = parser.http_should_keep_alive();
            Ok(CallbackDecision::Nothing)
        }
    }

    fn on_body(&mut self, parser : &mut HttpParser, data : &[u8]) -> CallbackResult {
        if self.paused {
            self.dontcall.on_body(parser, data)
        } else {
            parser.pause(true);
            self.paused = true;
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
            Ok(CallbackDecision::Nothing)
        }
    }

    fn on_message_complete(&mut self, parser : &mut HttpParser) -> CallbackResult {
        if self.paused {
            self.dontcall.on_message_complete(parser)
        } else {
            parser.pause(true);
            self.paused = true;
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

            Ok(CallbackDecision::Nothing)
        }
    }
}

pub struct CallbackCountBody {
    pub num_messages: usize, // maybe not necessary
    pub messages: Vec<Message>,
    pub currently_parsing_eof: bool,
}

impl Default for CallbackCountBody {
    fn default() -> CallbackCountBody {
        CallbackCountBody {
            num_messages: 0,
            messages: Vec::new(),
            currently_parsing_eof: false,
        }
    }
}

// find a way to reuse the code in CallbackRegular
impl HttpParserCallback for CallbackCountBody {
    fn on_message_begin(&mut self, _ : &mut HttpParser) -> CallbackResult {
        self.messages[self.num_messages].message_begin_cb_called = true;
        Ok(CallbackDecision::Nothing)
    }

    fn on_url(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
        for b in data {
            self.messages[self.num_messages].request_url.push(*b);
        }
        Ok(CallbackDecision::Nothing)
    }

    fn on_status(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
        for b in data {
            self.messages[self.num_messages].response_status.push(*b);
        }
        Ok(CallbackDecision::Nothing)
    }

    fn on_header_field(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
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

        Ok(CallbackDecision::Nothing)
    }

    fn on_header_value(&mut self, _ : &mut HttpParser, data : &[u8]) -> CallbackResult {
        let m : &mut Message = &mut self.messages[self.num_messages];

        match str::from_utf8(data) {
            Result::Ok(data_str) => {
                let i = m.headers.len()-1;
                m.headers[i][1].push_str(data_str);
            },
            _ => panic!("on_header_value: data is not in utf8 encoding"),
        }

        m.last_header_element = LastHeaderType::Value;

        Ok(CallbackDecision::Nothing)
    }

    fn on_headers_complete(&mut self, parser : &mut HttpParser) -> CallbackResult {
        let m : &mut Message = &mut self.messages[self.num_messages];
        m.method = parser.method;
        m.status_code = parser.status_code;
        m.http_version = parser.http_version;
        m.headers_complete_cb_called = true;
        m.should_keep_alive = parser.http_should_keep_alive();
        Ok(CallbackDecision::Nothing)
    }

    fn on_body(&mut self, parser : &mut HttpParser, data : &[u8]) -> CallbackResult {
        let m : &mut Message = &mut self.messages[self.num_messages];

        m.body_size += data.len();

        if m.body_is_final {
            panic!("\n\n ** Error http_body_is_final() should return 1 \
                    on last on_body callback call \
                    but it doesn't! **\n\n");
        }

        m.body_is_final = parser.http_body_is_final();
        Ok(CallbackDecision::Nothing)
    }

    fn on_message_complete(&mut self, parser : &mut HttpParser) -> CallbackResult {
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

        Ok(CallbackDecision::Nothing)
    }
}
pub fn print_error(errno: HttpErrno, raw: &[u8], error_location: usize) {
    println!("\n*** {} ***\n", errno);

    let len = raw.len();
    let mut this_line = false;
    let mut char_len: u64;
    let mut error_location_line = 0;
    let mut eof = true;
    for i in (0..len ) {
        if i == (error_location as usize) { this_line = true; }
        match raw[i] {
            b'\r' => {
                char_len = 2;
                print!("\\r");
            },
            b'\n' => {
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
                print!("{}", (raw[i] as char));
            },
        }
        if !this_line { error_location_line += char_len; }       
    }

    if eof {
        println!("[eof]");
    }

    for _ in (0..error_location_line) {
        print!(" ");
    }
    println!("^\n\nerror location: {}", error_location);
}

pub fn assert_eq_message(actual: &Message, expected: &Message) {
    assert_eq!(actual.http_version, expected.http_version);

    if expected.tp == HttpParserType::Request {
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

    for i in (0..actual.num_headers) {
        assert_eq!(actual.headers[i as usize][0], expected.headers[i as usize][0]);
        assert_eq!(actual.headers[i as usize][1], expected.headers[i as usize][1]);
    }

    assert_eq!(actual.upgrade, expected.upgrade);
}

pub fn test_message(message: &Message) {
    let raw = &message.raw;
    let raw_len = raw.len();
    for i in (0..raw_len) {
        let mut hp = HttpParser::new(message.tp);
        hp.strict = message.strict;

        let mut cb = CallbackRegular{..Default::default()};
        cb.messages.push(Message{..Default::default()});

        let mut read: usize;

        if i > 0 {
            read = hp.execute(&mut cb, &raw.as_bytes()[0 .. i]);

            if message.upgrade.is_some() && hp.upgrade {
                cb.messages[cb.num_messages - 1].upgrade = Some(raw[read..].to_string());
                assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", message.name);
                assert_eq_message(&cb.messages[0], message);
                continue;
            }

            if read != i {
                print_error(hp.errno.unwrap(), raw.as_bytes(), read);
                panic!();
            }
        }

        read = hp.execute(&mut cb, &raw.as_bytes()[i..]);

        if message.upgrade.is_some() && hp.upgrade {
            cb.messages[cb.num_messages - 1].upgrade = Some(raw[i+read..].to_string());
            assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", message.name);
            assert_eq_message(&cb.messages[0], message);
            continue;
        }

        if read != (raw_len - i) {
            print_error(hp.errno.unwrap(), raw.as_bytes(), i + read);
            panic!();
        }

        cb.currently_parsing_eof = true;
        read = hp.execute(&mut cb, &[]);

        if read != 0 {
            print_error(hp.errno.unwrap(), raw.as_bytes(), read);
            panic!();
        }

        assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", message.name);
        assert_eq_message(&cb.messages[0], message);
    }
}

pub fn test_message_pause(msg: &Message) {
    let mut raw : &str = &msg.raw;

    let mut hp = HttpParser::new(msg.tp);
    hp.strict = msg.strict;

    let mut cb = CallbackPause{..Default::default()};
    cb.messages.push(Message{..Default::default()});

    while raw.len() > 0 {
        cb.paused = false;
        let read = hp.execute(&mut cb, raw.as_bytes());

        if cb.messages[0].message_complete_cb_called &&
            msg.upgrade.is_some() && hp.upgrade {
            cb.messages[0].upgrade = Some(raw[read..].to_string());
            assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", msg.name);
            assert_eq_message(&cb.messages[0], msg);
            return;
        }

        if read < raw.len() {
            if hp.errno == Option::Some(HttpErrno::Strict) {
                return;
            }

            assert!(hp.errno == Option::Some(HttpErrno::Paused));
        }

        raw = &raw[read..];
        hp.pause(false);
    }

    cb.currently_parsing_eof = true;
    cb.paused = false;
    let read = hp.execute(&mut cb, &[]);
    assert_eq!(read, 0);

    assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", msg.name);
    assert_eq_message(&cb.messages[0], msg);
}

fn count_parsed_messages(messages: &[&Message]) -> usize {
    let mut i: usize = 0;
    let len = messages.len();

    while i < len {
        let msg = messages[i];
        i += 1;

        if msg.upgrade.is_some() {
            break;
        }
    }

    i
}

pub fn test_multiple3(r1: &Message, r2: &Message, r3: &Message) {
    let messages = [r1, r2, r3];
    let message_count = count_parsed_messages(&messages);

    let mut total = String::new();
    total.push_str(&r1.raw);
    total.push_str(&r2.raw);
    total.push_str(&r3.raw);

    let mut hp = HttpParser::new(r1.tp);
    hp.strict = r1.strict && r2.strict && r3.strict;

    let mut cb = CallbackRegular{..Default::default()};
    cb.messages.push(Message{..Default::default()});

    let mut read = hp.execute(&mut cb, total.as_bytes());

    if hp.upgrade {
        upgrade_message_fix(&mut cb, &total, read, &messages);

        assert!(message_count == cb.num_messages,
                "\n\n*** Parser didn't see 3 messages only {} *** \n", cb.num_messages);
        assert_eq_message(&cb.messages[0], r1);
        if message_count > 1 {
            assert_eq_message(&cb.messages[1], r2);
        }
        if message_count > 2 {
            assert_eq_message(&cb.messages[2], r3);
        }
        return;
    }

    if read != total.len() {
        print_error(hp.errno.unwrap(), total.as_bytes(), read);
        panic!();
    }

    cb.currently_parsing_eof = true;
    read = hp.execute(&mut cb, &[]);

    if read != 0 {
        print_error(hp.errno.unwrap(), total.as_bytes(), read);
        panic!();
    }

    assert!(message_count == cb.num_messages,
            "\n\n*** Parser didn't see 3 messages only {} *** \n", cb.num_messages);
    assert_eq_message(&cb.messages[0], r1);
    if message_count > 1 {
        assert_eq_message(&cb.messages[1], r2);
    }
    if message_count > 2 {
        assert_eq_message(&cb.messages[2], r3);
    }
}

fn upgrade_message_fix(cb: &mut CallbackRegular, body: &str, read: usize, msgs: &[&Message]) {
    let mut off : usize = 0;

    for m in msgs.iter() {
        off += m.raw.len();

        if m.upgrade.is_some() {
            let upgrade_len = m.upgrade.as_ref().unwrap().len();

            off -= upgrade_len;

            assert_eq!(&body[off..], &body[read..]);

            cb.messages[cb.num_messages-1].upgrade = 
                Some(body[read .. read+upgrade_len].to_string());
            return;
        }
    }

    panic!("\n\n*** Error: expected a message with upgrade ***\n");
}

fn print_test_scan_error(i: usize, j: usize, buf1: &[u8], buf2: &[u8], buf3: &[u8]) {
    print!("i={}  j={}\n", i, j);
    unsafe {
        print!("buf1 ({}) {}\n\n", buf1.len(), str::from_utf8_unchecked(buf1));
        print!("buf2 ({}) {}\n\n", buf2.len(), str::from_utf8_unchecked(buf2));
        print!("buf3 ({}) {}\n\n", buf3.len(), str::from_utf8_unchecked(buf3));
    }
    panic!();
}

pub fn test_scan(r1: &Message, r2: &Message, r3: &Message) {
    let mut total = String::new();
    total.push_str(&r1.raw);
    total.push_str(&r2.raw);
    total.push_str(&r3.raw);

    let total_len = total.len();

    let message_count = count_parsed_messages(&[r1, r2, r3]);

    for &is_type_both in [false, true].iter() {
        for j in (2..total_len) {
            for i in (1..j) {
                let mut hp = HttpParser::new(if is_type_both { HttpParserType::Both } else { r1.tp });
                hp.strict = r1.strict && r2.strict && r3.strict;

                let mut cb = CallbackRegular{..Default::default()};
                cb.messages.push(Message{..Default::default()});

                let mut done = false;
                
                let buf1 = &total.as_bytes()[0 .. i];
                let buf2 = &total.as_bytes()[i .. j];
                let buf3 = &total.as_bytes()[j .. total_len];

                let mut read = hp.execute(&mut cb, buf1);

                if hp.upgrade {
                    done = true;
                } else {
                    if read != buf1.len() {
                        print_error(hp.errno.unwrap(), buf1, read);
                        print_test_scan_error(i, j, buf1, buf2, buf3);
                    }
                }

                if !done {
                    read += hp.execute(&mut cb, buf2);

                    if hp.upgrade {
                        done = true;
                    } else {
                        if read != (buf1.len() + buf2.len()) {
                            print_error(hp.errno.unwrap(), buf2, read);
                            print_test_scan_error(i, j, buf1, buf2, buf3);
                        }
                    }
                }

                if !done {
                    read += hp.execute(&mut cb, buf3);

                    if hp.upgrade {
                        done = true;
                    } else {
                        if read != (buf1.len() + buf2.len() + buf3.len()) {
                            print_error(hp.errno.unwrap(), buf3, read);
                            print_test_scan_error(i, j, buf1, buf2, buf3);
                        }
                    }
                }

                if !done {
                    cb.currently_parsing_eof = true;
                    read = hp.execute(&mut cb, &[]);
                }

                // test

                if hp.upgrade {
                    upgrade_message_fix(&mut cb, &total, read, &[r1, r2, r3]);
                }

                if message_count != cb.num_messages {
                    print!("\n\nParser didn't see {} messages only {}\n", message_count, cb.num_messages);
                    print_test_scan_error(i, j, buf1, buf2, buf3);
                }

                assert_eq_message(&cb.messages[0], r1);
                if message_count > 1 {
                    assert_eq_message(&cb.messages[1], r2);
                }
                if message_count > 2 {
                    assert_eq_message(&cb.messages[2], r3);
                }
            }
        }
    }
}
