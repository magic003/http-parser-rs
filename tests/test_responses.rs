extern crate http_parser;

use std::default::Default;

use http_parser::{HttpParser, HttpParserType};

mod helper;

#[test]
fn test_responses() {
    // RESPONSES
    let responses: [helper::Message, ..1] = [
        helper::Message {
            name: String::from_str("google 301"),
            tp: HttpParserType::HttpResponse,
            raw: String::from_str(
                "HTTP/1.1 301 Moved Permanently\r\n\
                Location: http://www.google.com/\r\n\
                Content-Type: text/html; charset=UTF-8\r\n\
                Date: Sun, 26 Apr 2009 11:11:49 GMT\r\n\
                Expires: Tue, 26 May 2009 11:11:49 GMT\r\n\
                X-$PrototypeBI-Version: 1.6.0.3\r\n\
                Cache-Control: public, max-age=2592000\r\n\
                Server: gws\r\n\
                Content-Length: 219 \r\n\
                \r\n\
                <HTML><HEAD><meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\">\n\
                <TITLE>301 Moved</TITLE></HEAD><BODY>\n\
                <H1>301 Moved</H1>\n\
                The document has moved\n\
                <A HREF=\"http://www.google.com/\">here</A>.\r\n\
                </BODY></HTML>\r\n"),
            should_keep_alive: true,
            http_major: 1,
            http_minor: 1,
            status_code: 301,
            response_status: String::from_str("Moved Permanently"),
            num_headers: 8,
            headers: vec![
                [ String::from_str("Location"), String::from_str("http://www.google.com/") ],
                [ String::from_str("Content-Type"), String::from_str("text/html; charset=UTF-8") ],
                [ String::from_str("Date"), String::from_str("Sun, 26 Apr 2009 11:11:49 GMT") ],
                [ String::from_str("Expires"), String::from_str("Tue, 26 May 2009 11:11:49 GMT") ],
                [ String::from_str("X-$PrototypeBI-Version"), String::from_str("1.6.0.3") ],
                [ String::from_str("Cache-Control"), String::from_str("public, max-age=2592000") ],
                [ String::from_str("Server"), String::from_str("gws") ],
                [ String::from_str("Content-Length"), String::from_str("219 ") ],
            ],
            body: String::from_str("<HTML><HEAD><meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\">\n\
                                    <TITLE>301 Moved</TITLE></HEAD><BODY>\n\
                                    <H1>301 Moved</H1>\n\
                                    The document has moved\n\
                                    <A HREF=\"http://www.google.com/\">here</A>.\r\n\
                                    </BODY></HTML>\r\n"),
            ..Default::default()
        },
    ];
    // End of RESPONSES
    for m in responses.iter() {
        test_message(m);
    }
}

fn test_message(message: &helper::Message) {
    let raw = &message.raw;
    let raw_len = raw.len();
    for i in range(0, raw_len) {
        let mut hp = HttpParser::new(message.tp);
        let mut cb = helper::CallbackRegular{..Default::default()};
        cb.messages.push(helper::Message{..Default::default()});
        let mut read: u64 = 0;

        if i > 0 {
            read = hp.execute(&mut cb, raw.slice(0, i).as_bytes());

            if !message.upgrade.is_empty() && hp.upgrade {
                cb.messages[cb.num_messages - 1].upgrade = raw.slice_from(read as uint).to_string();
                assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", message.name);
                helper::assert_eq_message(&cb.messages[0], message);
                continue;
            }

            if read != (i as u64) {
                helper::print_error(hp.errno, raw.as_slice(), read);
                panic!();
            }
        }

        read = hp.execute(&mut cb, raw.slice_from(i).as_bytes());

        if !(message.upgrade.is_empty()) && hp.upgrade {
            cb.messages[cb.num_messages - 1].upgrade = raw.slice_from(i+(read as uint)).to_string();
            assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", message.name);
            helper::assert_eq_message(&cb.messages[0], message);
            continue;
        }

        if read != ((raw_len - i) as u64) {
            helper::print_error(hp.errno, raw.as_slice(), (i + read as uint) as u64);
            panic!();
        }

        read = hp.execute(&mut cb, &[]);

        if (read != 0) {
            helper::print_error(hp.errno, raw.as_slice(), read);
            panic!();
        }

        assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", message.name);
        helper::assert_eq_message(&cb.messages[0], message);
    }
}
