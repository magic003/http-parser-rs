extern crate http_parser;

use std::default::Default;

use http_parser::{HttpParser, HttpParserType, HttpErrno, HttpMethod, HttpVersion};

mod helper;

#[test]
fn test_requests() {
    test_simple("GET / HTP/1.1\r\n\r\n", Option::Some(HttpErrno::InvalidVersion));

    // Well-formed but incomplete
    test_simple("GET / HTTP/1.1\r\n\
                 Content-Type: text/plain\r\n\
                 Content-Length: 6\r\n\
                 \r\n\
                 fooba", Option::None);

    let all_methods = [
        "DELETE",
        "GET",
        "HEAD",
        "POST",
        "PUT",
        // "CONNECT", // CONNECT can't be tested like other methods, it's a tunnel
        "OPTIONS",
        "TRACE",
        "COYP",
        "LOCK",
        "MKCOL",
        "MOVE",
        "PROPFIND",
        "PROPPATCH",
        "UNLOCK",
        "REPORT",
        "MKACTIVITY",
        "CHECKOUT",
        "MERGE",
        "M-SEARCH",
        "NOTIFY",
        "SUBSCRIBE",
        "PATCH",
    ];

    for &method in all_methods.iter() {
        let mut buf = String::new();
        buf.push_str(method);
        buf.push_str(" / HTTP1.1\r\n\r\n");

        test_simple(buf.as_slice(), Option::None);
    }

    let bad_methods = [
        "ASDF",
        "C******",
        "COLA",
        "GEM",
        "GETA",
        "M****",
        "MKCOLA",
        "PROPPATCHA",
        "PUN",
        "PX",
        "SA",
        "hello world",
    ];
    for &method in bad_methods.iter() {
        let mut buf = String::new();
        buf.push_str(method);
        buf.push_str(" / HTTP1.1\r\n\r\n");

        test_simple(buf.as_slice(), Option::Some(HttpErrno::InvalidMethod));
    }

    // illegal header field name line folding
    test_simple("GET / HTTP/1.1\r\n\
                 name\r\n\
                  :value\r\n\
                 \r\n", Option::Some(HttpErrno::InvalidHeaderToken));

    let dumbfuck2 = 
        "GET / HTTP/1.1\r\n\
        X-SSL-Bullshit: -----BEGIN CERTIFICATE-----\r\n\
        \tMIIFbTCCBFWgAwIBAgICH4cwDQYJKoZIhvcNAQEFBQAwcDELMAkGA1UEBhMCVUsx\r\n\
        \tETAPBgNVBAoTCGVTY2llbmNlMRIwEAYDVQQLEwlBdXRob3JpdHkxCzAJBgNVBAMT\r\n\
        \tAkNBMS0wKwYJKoZIhvcNAQkBFh5jYS1vcGVyYXRvckBncmlkLXN1cHBvcnQuYWMu\r\n\
        \tdWswHhcNMDYwNzI3MTQxMzI4WhcNMDcwNzI3MTQxMzI4WjBbMQswCQYDVQQGEwJV\r\n\
        \tSzERMA8GA1UEChMIZVNjaWVuY2UxEzARBgNVBAsTCk1hbmNoZXN0ZXIxCzAJBgNV\r\n\
        \tBAcTmrsogriqMWLAk1DMRcwFQYDVQQDEw5taWNoYWVsIHBhcmQYJKoZIhvcNAQEB\r\n\
        \tBQADggEPADCCAQoCggEBANPEQBgl1IaKdSS1TbhF3hEXSl72G9J+WC/1R64fAcEF\r\n\
        \tW51rEyFYiIeZGx/BVzwXbeBoNUK41OK65sxGuflMo5gLflbwJtHBRIEKAfVVp3YR\r\n\
        \tgW7cMA/s/XKgL1GEC7rQw8lIZT8RApukCGqOVHSi/F1SiFlPDxuDfmdiNzL31+sL\r\n\
        \t0iwHDdNkGjy5pyBSB8Y79dsSJtCW/iaLB0/n8Sj7HgvvZJ7x0fr+RQjYOUUfrePP\r\n\
        \tu2MSpFyf+9BbC/aXgaZuiCvSR+8Snv3xApQY+fULK/xY8h8Ua51iXoQ5jrgu2SqR\r\n\
        \twgA7BUi3G8LFzMBl8FRCDYGUDy7M6QaHXx1ZWIPWNKsCAwEAAaOCAiQwggIgMAwG\r\n\
        \tA1UdEwEB/wQCMAAwEQYJYIZIAYb4QgHTTPAQDAgWgMA4GA1UdDwEB/wQEAwID6DAs\r\n\
        \tBglghkgBhvhCAQ0EHxYdVUsgZS1TY2llbmNlIFVzZXIgQ2VydGlmaWNhdGUwHQYD\r\n\
        \tVR0OBBYEFDTt/sf9PeMaZDHkUIldrDYMNTBZMIGaBgNVHSMEgZIwgY+AFAI4qxGj\r\n\
        \tloCLDdMVKwiljjDastqooXSkcjBwMQswCQYDVQQGEwJVSzERMA8GA1UEChMIZVNj\r\n\
        \taWVuY2UxEjAQBgNVBAsTCUF1dGhvcml0eTELMAkGA1UEAxMCQ0ExLTArBgkqhkiG\r\n\
        \t9w0BCQEWHmNhLW9wZXJhdG9yQGdyaWQtc3VwcG9ydC5hYy51a4IBADApBgNVHRIE\r\n\
        \tIjAggR5jYS1vcGVyYXRvckBncmlkLXN1cHBvcnQuYWMudWswGQYDVR0gBBIwEDAO\r\n\
        \tBgwrBgEEAdkvAQEBAQYwPQYJYIZIAYb4QgEEBDAWLmh0dHA6Ly9jYS5ncmlkLXN1\r\n\
        \tcHBvcnQuYWMudmT4sopwqlBWsvcHViL2NybC9jYWNybC5jcmwwPQYJYIZIAYb4QgEDBDAWLmh0\r\n\
        \tdHA6Ly9jYS5ncmlkLXN1cHBvcnQuYWMudWsvcHViL2NybC9jYWNybC5jcmwwPwYD\r\n\
        \tVR0fBDgwNjA0oDKgMIYuaHR0cDovL2NhLmdyaWQt5hYy51ay9wdWIv\r\n\
        \tY3JsL2NhY3JsLmNybDANBgkqhkiG9w0BAQUFAAOCAQEAS/U4iiooBENGW/Hwmmd3\r\n\
        \tXCy6Zrt08YjKCzGNjorT98g8uGsqYjSxv/hmi0qlnlHs+k/3Iobc3LjS5AMYr5L8\r\n\
        \tUO7OSkgFFlLHQyC9JzPfmLCAugvzEbyv4Olnsr8hbxF1MbKZoQxUZtMVu29wjfXk\r\n\
        \thTeApBv7eaKCWpSp7MCbvgzm74izKhu3vlDk9w6qVrxePfGgpKPqfHiOoGhFnbTK\r\n\
        \twTC6o2xq5y0qZ03JonF7OJspEd3I5zKY3E+ov7/ZhW6DqT8UFvsAdjvQbXyhV8Eu\r\n\
        \tYhixw1aKEPzNjNowuIseVogKOLXxWI5vAi5HgXdS0/ES5gDGsABo4fqovUKlgop3\r\n\
        \tRA==\r\n\
        \t-----END CERTIFICATE-----\r\n\
        \r\n";
    test_simple(dumbfuck2, Option::None);

    // REQUESTS
    let requests = [
        helper::Message {
            name: String::from_str("curl get"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /test HTTP/1.1\r\n\
                User-Agent: curl/7.18.0 (i486-pc-linux-gnu) libcurl/7.18.0 OpenSSL/0.9.8g zlib/1.2.3.3 libidn/1.1\r\n\
                Host: 0.0.0.0=5000\r\n\
                Accept: */*\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/test"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/test".as_bytes());
                v
            },
            num_headers: 3,
            headers: vec![
                [ String::from_str("User-Agent"), String::from_str("curl/7.18.0 (i486-pc-linux-gnu) libcurl/7.18.0 OpenSSL/0.9.8g zlib/1.2.3.3 libidn/1.1") ],
                [ String::from_str("Host"), String::from_str("0.0.0.0=5000") ],
                [ String::from_str("Accept"), String::from_str("*/*") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("firefox get"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /favicon.ico HTTP/1.1\r\n\
                Host: 0.0.0.0=5000\r\n\
                User-Agent: Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.9) Gecko/2008061015 Firefox/3.0\r\n\
                Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
                Accept-Language: en-us,en;q=0.5\r\n\
                Accept-Encoding: gzip,deflate\r\n\
                Accept-Charset: ISO-8859-1,utf-8;q=0.7,*;q=0.7\r\n\
                Keep-Alive: 300\r\n\
                Connection: keep-alive\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/favicon.ico"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/favicon.ico".as_bytes());
                v
            },
            num_headers: 8,
            headers: vec![
                [ String::from_str("Host"), String::from_str("0.0.0.0=5000") ],
                [ String::from_str("User-Agent"), String::from_str("Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.9) Gecko/2008061015 Firefox/3.0") ],
                [ String::from_str("Accept"), String::from_str("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8") ],
                [ String::from_str("Accept-Language"), String::from_str("en-us,en;q=0.5") ],
                [ String::from_str("Accept-Encoding"), String::from_str("gzip,deflate") ],
                [ String::from_str("Accept-Charset"), String::from_str("ISO-8859-1,utf-8;q=0.7,*;q=0.7") ],
                [ String::from_str("Keep-Alive"), String::from_str("300") ],
                [ String::from_str("Connection"), String::from_str("keep-alive") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("dumbfuck"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /dumbfuck HTTP/1.1\r\n\
                aaaaaaaaaaaaa: ++++++++++\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/dumbfuck"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/dumbfuck".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("aaaaaaaaaaaaa"), String::from_str("++++++++++") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("fragment in url"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /forums/1/topics/2375?page=1#posts-17408 HTTP/1.1\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str("page=1"),
            fragment: String::from_str("posts-17408"),
            request_path: String::from_str("/forums/1/topics/2375"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/forums/1/topics/2375?page=1#posts-17408".as_bytes());
                v
            },
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("get no headers no body"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /get_no_headers_no_body/world HTTP/1.1\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/get_no_headers_no_body/world"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/get_no_headers_no_body/world".as_bytes());
                v
            },
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("get one header no body"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /get_one_header_no_body/world HTTP/1.1\r\n\
                Accept: */*\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/get_one_header_no_body/world"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/get_one_header_no_body/world".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Accept"), String::from_str("*/*") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("get funky content length body hello"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /get_funky_content_length_body_hello HTTP/1.0\r\n\
                conTENT-Length: 5\r\n\
                \r\n\
                HELLO"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/get_funky_content_length_body_hello"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/get_funky_content_length_body_hello".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("conTENT-Length"), String::from_str("5") ]
            ],
            body: String::from_str("HELLO"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("post - chunked body: all your base are belong to us"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "POST /post_chunked_all_your_base HTTP/1.1\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                1e\r\nall your base are belong to us\r\n\
                0\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/post_chunked_all_your_base"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/post_chunked_all_your_base".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Transfer-Encoding"), String::from_str("chunked") ]
            ],
            body: String::from_str("all your base are belong to us"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("two chunks ; triple zero ending"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "POST /two_chunks_mult_zero_end HTTP/1.1\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                5\r\nhello\r\n\
                6\r\n world\r\n\
                000\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/two_chunks_mult_zero_end"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/two_chunks_mult_zero_end".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Transfer-Encoding"), String::from_str("chunked") ]
            ],
            body: String::from_str("hello world"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("chunked with trailing headers. blech."),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "POST /chunked_w_trailing_headers HTTP/1.1\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                5\r\nhello\r\n\
                6\r\n world\r\n\
                0\r\n\
                Vary: *\r\n\
                Content-Type: text/plain\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/chunked_w_trailing_headers"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/chunked_w_trailing_headers".as_bytes());
                v
            },
            num_headers: 3,
            headers: vec![
                [ String::from_str("Transfer-Encoding"), String::from_str("chunked") ],
                [ String::from_str("Vary"), String::from_str("*") ],
                [ String::from_str("Content-Type"), String::from_str("text/plain") ],
            ],
            body: String::from_str("hello world"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("with bullshit after the length"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "POST /chunked_w_bullshit_after_length HTTP/1.1\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                5; ihatew3;whatthefuck=aretheseparametersfor\r\nhello\r\n\
                6; blahblah; blah\r\n world\r\n\
                0\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/chunked_w_bullshit_after_length"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/chunked_w_bullshit_after_length".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Transfer-Encoding"), String::from_str("chunked") ],
            ],
            body: String::from_str("hello world"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("with quotes"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /with_\"stupid\"_quotes?foo=\"bar\" HTTP/1.1\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str("foo=\"bar\""),
            fragment: String::from_str(""),
            request_path: String::from_str("/with_\"stupid\"_quotes"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/with_\"stupid\"_quotes?foo=\"bar\"".as_bytes());
                v
            },
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("apachebench get"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /test HTTP/1.0\r\n\
                Host: 0.0.0.0:5000\r\n\
                User-Agent: ApacheBench/2.3\r\n\
                Accept: */*\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/test"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/test".as_bytes());
                v
            },
            num_headers: 3,
            headers: vec![
                [ String::from_str("Host"), String::from_str("0.0.0.0:5000") ],
                [ String::from_str("User-Agent"), String::from_str("ApacheBench/2.3") ],
                [ String::from_str("Accept"), String::from_str("*/*") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("query url with question mark"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /test.cgi?foo=bar?baz HTTP/1.1\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str("foo=bar?baz"),
            fragment: String::from_str(""),
            request_path: String::from_str("/test.cgi"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/test.cgi?foo=bar?baz".as_bytes());
                v
            },
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("newline prefix get"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "\r\nGET /test HTTP/1.1\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/test"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/test".as_bytes());
                v
            },
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("upgrade request"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /demo HTTP/1.1\r\n\
                Host: example.com\r\n\
                Connection: Upgrade\r\n\
                Sec-WebSocket-Key2: 12998 5 Y3 1  .P00\r\n\
                Sec-WebSocket-Protocol: sample\r\n\
                Upgrade: WebSocket\r\n\
                Sec-WebSocket-Key1: 4 @1  46546xW%0l 1 5\r\n\
                Origin: http://example.com\r\n\
                \r\n\
                Hot diggity dogg"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/demo"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/demo".as_bytes());
                v
            },
            num_headers: 7,
            upgrade: Some(String::from_str("Hot diggity dogg")),
            headers: vec![
                [ String::from_str("Host"), String::from_str("example.com") ],
                [ String::from_str("Connection"), String::from_str("Upgrade") ],
                [ String::from_str("Sec-WebSocket-Key2"), String::from_str("12998 5 Y3 1  .P00") ],
                [ String::from_str("Sec-WebSocket-Protocol"), String::from_str("sample") ],
                [ String::from_str("Upgrade"), String::from_str("WebSocket") ],
                [ String::from_str("Sec-WebSocket-Key1"), String::from_str("4 @1  46546xW%0l 1 5") ],
                [ String::from_str("Origin"), String::from_str("http://example.com") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("connect request"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "CONNECT 0-home0.netscape.com:443 HTTP/1.0\r\n\
                User-agent: Mozilla/1.1N\r\n\
                Proxy-authorization: basic aGVsbG86d29ybGQ=\r\n\
                \r\n\
                some data\r\n\
                and yet even more data"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            method: HttpMethod::Connect,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str(""),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("0-home0.netscape.com:443".as_bytes());
                v
            },
            num_headers: 2,
            upgrade: Some(String::from_str("some data\r\nand yet even more data")),
            headers: vec![
                [ String::from_str("User-agent"), String::from_str("Mozilla/1.1N") ],
                [ String::from_str("Proxy-authorization"), String::from_str("basic aGVsbG86d29ybGQ=") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("report request"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "REPORT /test HTTP/1.1\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Report,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/test"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/test".as_bytes());
                v
            },
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("request with no http version"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 0, minor: 9 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/".as_bytes());
                v
            },
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("m-search request"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "M-SEARCH * HTTP/1.1\r\n\
                HOST: 239.255.255.250:1900\r\n\
                MAN: \"ssdp:discover\"\r\n\
                ST: \"ssdp:all\"\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::MSearch,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("*"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("*".as_bytes());
                v
            },
            num_headers: 3,
            headers: vec![
                [ String::from_str("HOST"), String::from_str("239.255.255.250:1900")],
                [ String::from_str("MAN"), String::from_str("\"ssdp:discover\"")],
                [ String::from_str("ST"), String::from_str("\"ssdp:all\"")],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("line folding in header value"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET / HTTP/1.1\r\n\
                Line1:    abc\r\n\tdef\r\n ghi\r\n\t\tjkl\r\n  mno \r\n\t \tqrs\r\n\
                Line2: \t line2\t\r\n\
                Line3:\r\n line3\r\n\
                Line4: \r\n \r\n\
                Connection:\r\n close\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/".as_bytes());
                v
            },
            num_headers: 5,
            headers: vec![
                [ String::from_str("Line1"), String::from_str("abc\tdef ghi\t\tjkl  mno \t \tqrs")],
                [ String::from_str("Line2"), String::from_str("line2\t")],
                [ String::from_str("Line3"), String::from_str("line3")],
                [ String::from_str("Line4"), String::from_str("")],
                [ String::from_str("Connection"), String::from_str("close")],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("host terminated by a query string"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET http://hypnotoad.org?hail=all HTTP/1.1\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str("hail=all"),
            fragment: String::from_str(""),
            request_path: String::from_str(""),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("http://hypnotoad.org?hail=all".as_bytes());
                v
            },
            host: String::from_str("hypnotoad.org"),
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("host:port terminated by a query string"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET http://hypnotoad.org:1234?hail=all HTTP/1.1\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str("hail=all"),
            fragment: String::from_str(""),
            request_path: String::from_str(""),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("http://hypnotoad.org:1234?hail=all".as_bytes());
                v
            },
            host: String::from_str("hypnotoad.org"),
            port: 1234,
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("host:port terminated by a space"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET http://hypnotoad.org:1234 HTTP/1.1\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str(""),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("http://hypnotoad.org:1234".as_bytes());
                v
            },
            host: String::from_str("hypnotoad.org"),
            port: 1234,
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("PATCH request"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "PATCH /file.txt HTTP/1.1\r\n\
                Host: www.example.com\r\n\
                Content-Type: application/example\r\n\
                If-Match: \"e0023aa4e\"\r\n\
                Content-Length: 10\r\n\
                \r\n\
                cccccccccc"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Patch,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/file.txt"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/file.txt".as_bytes());
                v
            },
            num_headers: 4,
            headers: vec![
                [ String::from_str("Host"), String::from_str("www.example.com") ],
                [ String::from_str("Content-Type"), String::from_str("application/example") ],
                [ String::from_str("If-Match"), String::from_str("\"e0023aa4e\"") ],
                [ String::from_str("Content-Length"), String::from_str("10") ],
            ],
            body: String::from_str("cccccccccc"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("connect caps request"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "CONNECT HOME0.NETSCAPE.COM:443 HTTP/1.0\r\n\
                User-agent: Mozilla/1.1N\r\n\
                Proxy-authorization: basic aGVsbG86d29ybGQ=\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            method: HttpMethod::Connect,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str(""),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("HOME0.NETSCAPE.COM:443".as_bytes());
                v
            },
            num_headers: 2,
            upgrade: Some(String::from_str("")),
            headers: vec![
                [ String::from_str("User-agent"), String::from_str("Mozilla/1.1N") ],
                [ String::from_str("Proxy-authorization"), String::from_str("basic aGVsbG86d29ybGQ=") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("utf-8 path request"),
            tp: HttpParserType::Request,
            strict: false,
            raw: String::from_str( 
                "GET /δ¶/δt/pope?q=1#narf HTTP/1.1\r\n\
                Host: github.com\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str("q=1"),
            fragment: String::from_str("narf"),
            request_path: String::from_str("/δ¶/δt/pope"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/δ¶/δt/pope?q=1#narf".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Host"), String::from_str("github.com") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("hostname underscore"),
            tp: HttpParserType::Request,
            strict: false,
            raw: String::from_str( 
                "CONNECT home_0.netscape.com:443 HTTP/1.0\r\n\
                User-agent: Mozilla/1.1N\r\n\
                Proxy-authorization: basic aGVsbG86d29ybGQ=\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            method: HttpMethod::Connect,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str(""),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("home_0.netscape.com:443".as_bytes());
                v
            },
            num_headers: 2,
            upgrade: Some(String::new()),
            headers: vec![
                [ String::from_str("User-agent"), String::from_str("Mozilla/1.1N") ],
                [ String::from_str("Proxy-authorization"), String::from_str("basic aGVsbG86d29ybGQ=") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("eat CRLF between requests, no \"Connection: close\" header"),
            raw: String::from_str( 
                "POST / HTTP/1.1\r\n\
                Host: www.example.com\r\n\
                Content-Type: application/x-www-form-urlencoded\r\n\
                Content-Length: 4\r\n\
                \r\n\
                q=42\r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/".as_bytes());
                v
            },
            num_headers: 3,
            headers: vec![
                [ String::from_str("Host"), String::from_str("www.example.com") ],
                [ String::from_str("Content-Type"), String::from_str("application/x-www-form-urlencoded") ],
                [ String::from_str("Content-Length"), String::from_str("4") ],
            ],
            body: String::from_str("q=42"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("eat CRLF between requests even if \"Connection: close\" is set"),
            raw: String::from_str( 
                "POST / HTTP/1.1\r\n\
                Host: www.example.com\r\n\
                Content-Type: application/x-www-form-urlencoded\r\n\
                Content-Length: 4\r\n\
                Connection: close\r\n\
                \r\n\
                q=42\r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/".as_bytes());
                v
            },
            num_headers: 4,
            headers: vec![
                [ String::from_str("Host"), String::from_str("www.example.com") ],
                [ String::from_str("Content-Type"), String::from_str("application/x-www-form-urlencoded") ],
                [ String::from_str("Content-Length"), String::from_str("4") ],
                [ String::from_str("Connection"), String::from_str("close") ],
            ],
            body: String::from_str("q=42"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("PURGE request"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "PURGE /file.txt HTTP/1.1\r\n\
                Host: www.example.com\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Purge,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/file.txt"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/file.txt".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Host"), String::from_str("www.example.com") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("SEARCH request"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "SEARCH / HTTP/1.1\r\n\
                Host: www.example.com\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Search,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Host"), String::from_str("www.example.com") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("host:port and basic_auth"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET http://a%12:b!&*$@hypnotoad.org:1234/toto HTTP/1.1\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/toto"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("http://a%12:b!&*$@hypnotoad.org:1234/toto".as_bytes());
                v
            },
            host: String::from_str("hypnotoad.org"),
            userinfo: String::from_str("a%12:b!&*$"),
            port: 1234,
            num_headers: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("line folding in header value"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET / HTTP/1.1\r\n\
                Line1:    abc\n\tdef\n ghi\n\t\tjkl\n mno \n\t \tqrs\n\
                Line2: \t line2\t\n\
                Line3:\n line3\n\
                Line4: \n \n\
                Connection:\n close\n\
                \n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/".as_bytes());
                v
            },
            num_headers: 5,
            headers: vec![
                [ String::from_str("Line1"), String::from_str("abc\tdef ghi\t\tjkl mno \t \tqrs") ],
                [ String::from_str("Line2"), String::from_str("line2\t") ],
                [ String::from_str("Line3"), String::from_str("line3") ],
                [ String::from_str("Line4"), String::from_str("") ],
                [ String::from_str("Connection"), String::from_str("close") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("multiple connection header values with folding"),
            tp: HttpParserType::Request,
            raw: String::from_str( 
                "GET /demo HTTP/1.1\r\n\
                Host: example.com\r\n\
                Connection: Something,\r\n Upgrade, ,Keep-Alive\r\n\
                Sec-WebSocket-Key2: 12998 5 Y3 1  .P00\r\n\
                Sec-WebSocket-Protocol: sample\r\n\
                Upgrade: WebSocket\r\n\
                Sec-WebSocket-Key1: 4 @1  46546xW%0l 1 5\r\n\
                Origin: http://example.com\r\n\
                \r\n\
                Hot diggity dogg"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: String::from_str(""),
            fragment: String::from_str(""),
            request_path: String::from_str("/demo"),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("/demo".as_bytes());
                v
            },
            num_headers: 7,
            upgrade: Some(String::from_str("Hot diggity dogg")),
            headers: vec![
                [ String::from_str("Host"), String::from_str("example.com") ],
                [ String::from_str("Connection"), String::from_str("Something, Upgrade, ,Keep-Alive") ],
                [ String::from_str("Sec-WebSocket-Key2"), String::from_str("12998 5 Y3 1  .P00") ],
                [ String::from_str("Sec-WebSocket-Protocol"), String::from_str("sample") ],
                [ String::from_str("Upgrade"), String::from_str("WebSocket") ],
                [ String::from_str("Sec-WebSocket-Key1"), String::from_str("4 @1  46546xW%0l 1 5") ],
                [ String::from_str("Origin"), String::from_str("http://example.com") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
    ];

    const GET_NO_HEADERS_NO_BODY : uint = 4;
    const GET_ONE_HEADER_NO_BODY : uint = 5;
    const GET_FUNKY_CONTENT_LENGTH : uint = 6;
    const POST_IDENTITY_BODY_WORLD : uint = 7;
    const POST_CHUNKED_ALL_YOUR_BASE : uint = 8;
    const TWO_CHUNKS_MULT_ZERO_END : uint = 9;
    const CHUNKED_W_TRAILING_HEADERS : uint = 10;
    const CHUNKED_W_BULLSHIT_AFTER_LENGTH : uint = 11;
    const QUERY_URL_WITH_QUESTION_MARK_GET : uint = 14;
    const PREFIX_NEWLINE_GET : uint = 15;
    const CONNECT_REQUEST : uint = 17;

    // End REQUESTS

    for m in requests.iter() {
        helper::test_message(m);
    }
    for m in requests.iter() {
        helper::test_message_pause(m);
    }

    for r1 in requests.iter() {
        if !r1.should_keep_alive { continue; }
        for r2 in requests.iter() {
            if !r2.should_keep_alive { continue; }
            for r3 in requests.iter() {
                helper::test_multiple3(r1, r2, r3);
            }
        }
    }

    helper::test_scan(&requests[GET_NO_HEADERS_NO_BODY],
                      &requests[GET_ONE_HEADER_NO_BODY],
                      &requests[GET_NO_HEADERS_NO_BODY]);

    helper::test_scan(&requests[POST_CHUNKED_ALL_YOUR_BASE],
                      &requests[POST_IDENTITY_BODY_WORLD],
                      &requests[GET_FUNKY_CONTENT_LENGTH]);

    helper::test_scan(&requests[TWO_CHUNKS_MULT_ZERO_END],
                      &requests[CHUNKED_W_TRAILING_HEADERS],
                      &requests[CHUNKED_W_BULLSHIT_AFTER_LENGTH]);

    helper::test_scan(&requests[QUERY_URL_WITH_QUESTION_MARK_GET],
                      &requests[PREFIX_NEWLINE_GET],
                      &requests[CONNECT_REQUEST]);
}

fn test_simple(buf: &str, err_expected: Option<HttpErrno>) {
    let mut hp = HttpParser::new(HttpParserType::Request);

    let mut cb = helper::CallbackRegular{..Default::default()};
    cb.messages.push(helper::Message{..Default::default()});

    hp.execute(&mut cb, buf.as_bytes());
    let err = hp.errno;
    cb.currently_parsing_eof = true;
    hp.execute(&mut cb, &[]);

    assert!(err_expected == err || 
            (hp.strict && (err_expected.is_none() || err == Option::Some(HttpErrno::Strict))),
            "\n*** test_simple expected {}, but saw {} ***\n\n{}\n", 
            err_expected.unwrap().to_string(), err.unwrap().to_string(), buf);
}
