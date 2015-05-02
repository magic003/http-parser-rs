extern crate http_parser;

use std::default::Default;

use http_parser::{HttpParser, HttpParserType, HttpErrno, HttpMethod, HttpVersion};

pub mod helper;

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

        test_simple(&buf, Option::None);
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

        test_simple(&buf, Option::Some(HttpErrno::InvalidMethod));
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
            name: "curl get".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /test HTTP/1.1\r\n\
                User-Agent: curl/7.18.0 (i486-pc-linux-gnu) libcurl/7.18.0 OpenSSL/0.9.8g zlib/1.2.3.3 libidn/1.1\r\n\
                Host: 0.0.0.0=5000\r\n\
                Accept: */*\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/test".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/test".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 3,
            headers: vec![
                [ "User-Agent".to_string(), "curl/7.18.0 (i486-pc-linux-gnu) libcurl/7.18.0 OpenSSL/0.9.8g zlib/1.2.3.3 libidn/1.1".to_string() ],
                [ "Host".to_string(), "0.0.0.0=5000".to_string() ],
                [ "Accept".to_string(), "*/*".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "firefox get".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /favicon.ico HTTP/1.1\r\n\
                Host: 0.0.0.0=5000\r\n\
                User-Agent: Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.9) Gecko/2008061015 Firefox/3.0\r\n\
                Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
                Accept-Language: en-us,en;q=0.5\r\n\
                Accept-Encoding: gzip,deflate\r\n\
                Accept-Charset: ISO-8859-1,utf-8;q=0.7,*;q=0.7\r\n\
                Keep-Alive: 300\r\n\
                Connection: keep-alive\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/favicon.ico".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/favicon.ico".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 8,
            headers: vec![
                [ "Host".to_string(), "0.0.0.0=5000".to_string() ],
                [ "User-Agent".to_string(), "Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.9) Gecko/2008061015 Firefox/3.0".to_string() ],
                [ "Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string() ],
                [ "Accept-Language".to_string(), "en-us,en;q=0.5".to_string() ],
                [ "Accept-Encoding".to_string(), "gzip,deflate".to_string() ],
                [ "Accept-Charset".to_string(), "ISO-8859-1,utf-8;q=0.7,*;q=0.7".to_string() ],
                [ "Keep-Alive".to_string(), "300".to_string() ],
                [ "Connection".to_string(), "keep-alive".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "dumbfuck".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /dumbfuck HTTP/1.1\r\n\
                aaaaaaaaaaaaa: ++++++++++\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/dumbfuck".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/dumbfuck".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 1,
            headers: vec![
                [ "aaaaaaaaaaaaa".to_string(), "++++++++++".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "fragment in url".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /forums/1/topics/2375?page=1#posts-17408 HTTP/1.1\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "page=1".to_string(),
            fragment: "posts-17408".to_string(),
            request_path: "/forums/1/topics/2375".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/forums/1/topics/2375?page=1#posts-17408".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "get no headers no body".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /get_no_headers_no_body/world HTTP/1.1\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/get_no_headers_no_body/world".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/get_no_headers_no_body/world".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "get one header no body".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /get_one_header_no_body/world HTTP/1.1\r\n\
                Accept: */*\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/get_one_header_no_body/world".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/get_one_header_no_body/world".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 1,
            headers: vec![
                [ "Accept".to_string(), "*/*".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "get funky content length body hello".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /get_funky_content_length_body_hello HTTP/1.0\r\n\
                conTENT-Length: 5\r\n\
                \r\n\
                HELLO".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/get_funky_content_length_body_hello".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/get_funky_content_length_body_hello".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 1,
            headers: vec![
                [ "conTENT-Length".to_string(), "5".to_string() ]
            ],
            body: "HELLO".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "post - chunked body: all your base are belong to us".to_string(),
            tp: HttpParserType::Request,
            raw: "POST /post_chunked_all_your_base HTTP/1.1\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                1e\r\nall your base are belong to us\r\n\
                0\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/post_chunked_all_your_base".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/post_chunked_all_your_base".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 1,
            headers: vec![
                [ "Transfer-Encoding".to_string(), "chunked".to_string() ]
            ],
            body: "all your base are belong to us".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "two chunks ; triple zero ending".to_string(),
            tp: HttpParserType::Request,
            raw: "POST /two_chunks_mult_zero_end HTTP/1.1\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                5\r\nhello\r\n\
                6\r\n world\r\n\
                000\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/two_chunks_mult_zero_end".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/two_chunks_mult_zero_end".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 1,
            headers: vec![
                [ "Transfer-Encoding".to_string(), "chunked".to_string() ]
            ],
            body: "hello world".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "chunked with trailing headers. blech.".to_string(),
            tp: HttpParserType::Request,
            raw: "POST /chunked_w_trailing_headers HTTP/1.1\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                5\r\nhello\r\n\
                6\r\n world\r\n\
                0\r\n\
                Vary: *\r\n\
                Content-Type: text/plain\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/chunked_w_trailing_headers".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/chunked_w_trailing_headers".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 3,
            headers: vec![
                [ "Transfer-Encoding".to_string(), "chunked".to_string() ],
                [ "Vary".to_string(), "*".to_string() ],
                [ "Content-Type".to_string(), "text/plain".to_string() ],
            ],
            body: "hello world".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "with bullshit after the length".to_string(),
            tp: HttpParserType::Request,
            raw: "POST /chunked_w_bullshit_after_length HTTP/1.1\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                5; ihatew3;whatthefuck=aretheseparametersfor\r\nhello\r\n\
                6; blahblah; blah\r\n world\r\n\
                0\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/chunked_w_bullshit_after_length".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/chunked_w_bullshit_after_length".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 1,
            headers: vec![
                [ "Transfer-Encoding".to_string(), "chunked".to_string() ],
            ],
            body: "hello world".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "with quotes".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /with_\"stupid\"_quotes?foo=\"bar\" HTTP/1.1\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "foo=\"bar\"".to_string(),
            fragment: "".to_string(),
            request_path: "/with_\"stupid\"_quotes".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/with_\"stupid\"_quotes?foo=\"bar\"".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "apachebench get".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /test HTTP/1.0\r\n\
                Host: 0.0.0.0:5000\r\n\
                User-Agent: ApacheBench/2.3\r\n\
                Accept: */*\r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/test".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/test".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 3,
            headers: vec![
                [ "Host".to_string(), "0.0.0.0:5000".to_string() ],
                [ "User-Agent".to_string(), "ApacheBench/2.3".to_string() ],
                [ "Accept".to_string(), "*/*".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "query url with question mark".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /test.cgi?foo=bar?baz HTTP/1.1\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "foo=bar?baz".to_string(),
            fragment: "".to_string(),
            request_path: "/test.cgi".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/test.cgi?foo=bar?baz".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "newline prefix get".to_string(),
            tp: HttpParserType::Request,
            raw: "\r\nGET /test HTTP/1.1\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/test".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/test".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "upgrade request".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /demo HTTP/1.1\r\n\
                Host: example.com\r\n\
                Connection: Upgrade\r\n\
                Sec-WebSocket-Key2: 12998 5 Y3 1  .P00\r\n\
                Sec-WebSocket-Protocol: sample\r\n\
                Upgrade: WebSocket\r\n\
                Sec-WebSocket-Key1: 4 @1  46546xW%0l 1 5\r\n\
                Origin: http://example.com\r\n\
                \r\n\
                Hot diggity dogg".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/demo".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/demo".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 7,
            upgrade: Some("Hot diggity dogg".to_string()),
            headers: vec![
                [ "Host".to_string(), "example.com".to_string() ],
                [ "Connection".to_string(), "Upgrade".to_string() ],
                [ "Sec-WebSocket-Key2".to_string(), "12998 5 Y3 1  .P00".to_string() ],
                [ "Sec-WebSocket-Protocol".to_string(), "sample".to_string() ],
                [ "Upgrade".to_string(), "WebSocket".to_string() ],
                [ "Sec-WebSocket-Key1".to_string(), "4 @1  46546xW%0l 1 5".to_string() ],
                [ "Origin".to_string(), "http://example.com".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "connect request".to_string(),
            tp: HttpParserType::Request,
            raw: "CONNECT 0-home0.netscape.com:443 HTTP/1.0\r\n\
                User-agent: Mozilla/1.1N\r\n\
                Proxy-authorization: basic aGVsbG86d29ybGQ=\r\n\
                \r\n\
                some data\r\n\
                and yet even more data".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            method: HttpMethod::Connect,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "0-home0.netscape.com:443".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 2,
            upgrade: Some("some data\r\nand yet even more data".to_string()),
            headers: vec![
                [ "User-agent".to_string(), "Mozilla/1.1N".to_string() ],
                [ "Proxy-authorization".to_string(), "basic aGVsbG86d29ybGQ=".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "report request".to_string(),
            tp: HttpParserType::Request,
            raw: "REPORT /test HTTP/1.1\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Report,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/test".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/test".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "request with no http version".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /\r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 0, minor: 9 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push(b'/');
                v
            },
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "m-search request".to_string(),
            tp: HttpParserType::Request,
            raw: "M-SEARCH * HTTP/1.1\r\n\
                HOST: 239.255.255.250:1900\r\n\
                MAN: \"ssdp:discover\"\r\n\
                ST: \"ssdp:all\"\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::MSearch,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "*".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push(b'*');
                v
            },
            num_headers: 3,
            headers: vec![
                [ "HOST".to_string(), "239.255.255.250:1900".to_string()],
                [ "MAN".to_string(), "\"ssdp:discover\"".to_string()],
                [ "ST".to_string(), "\"ssdp:all\"".to_string()],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "line folding in header value".to_string(),
            tp: HttpParserType::Request,
            raw: "GET / HTTP/1.1\r\n\
                Line1:    abc\r\n\tdef\r\n ghi\r\n\t\tjkl\r\n  mno \r\n\t \tqrs\r\n\
                Line2: \t line2\t\r\n\
                Line3:\r\n line3\r\n\
                Line4: \r\n \r\n\
                Connection:\r\n close\r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push(b'/');
                v
            },
            num_headers: 5,
            headers: vec![
                [ "Line1".to_string(), "abc\tdef ghi\t\tjkl  mno \t \tqrs".to_string()],
                [ "Line2".to_string(), "line2\t".to_string()],
                [ "Line3".to_string(), "line3".to_string()],
                [ "Line4".to_string(), "".to_string()],
                [ "Connection".to_string(), "close".to_string()],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "host terminated by a query string".to_string(),
            tp: HttpParserType::Request,
            raw: "GET http://hypnotoad.org?hail=all HTTP/1.1\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "hail=all".to_string(),
            fragment: "".to_string(),
            request_path: "".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "http://hypnotoad.org?hail=all".as_bytes() {
                    v.push(*b);
                }
                v
            },
            host: "hypnotoad.org".to_string(),
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "host:port terminated by a query string".to_string(),
            tp: HttpParserType::Request,
            raw: "GET http://hypnotoad.org:1234?hail=all HTTP/1.1\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "hail=all".to_string(),
            fragment: "".to_string(),
            request_path: "".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "http://hypnotoad.org:1234?hail=all".as_bytes() {
                    v.push(*b);
                }
                v
            },
            host: "hypnotoad.org".to_string(),
            port: 1234,
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "host:port terminated by a space".to_string(),
            tp: HttpParserType::Request,
            raw: "GET http://hypnotoad.org:1234 HTTP/1.1\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "http://hypnotoad.org:1234".as_bytes() {
                    v.push(*b);
                }
                v
            },
            host: "hypnotoad.org".to_string(),
            port: 1234,
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "PATCH request".to_string(),
            tp: HttpParserType::Request,
            raw: "PATCH /file.txt HTTP/1.1\r\n\
                Host: www.example.com\r\n\
                Content-Type: application/example\r\n\
                If-Match: \"e0023aa4e\"\r\n\
                Content-Length: 10\r\n\
                \r\n\
                cccccccccc".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Patch,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/file.txt".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/file.txt".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 4,
            headers: vec![
                [ "Host".to_string(), "www.example.com".to_string() ],
                [ "Content-Type".to_string(), "application/example".to_string() ],
                [ "If-Match".to_string(), "\"e0023aa4e\"".to_string() ],
                [ "Content-Length".to_string(), "10".to_string() ],
            ],
            body: "cccccccccc".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "connect caps request".to_string(),
            tp: HttpParserType::Request,
            raw: "CONNECT HOME0.NETSCAPE.COM:443 HTTP/1.0\r\n\
                User-agent: Mozilla/1.1N\r\n\
                Proxy-authorization: basic aGVsbG86d29ybGQ=\r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            method: HttpMethod::Connect,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "HOME0.NETSCAPE.COM:443".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 2,
            upgrade: Some("".to_string()),
            headers: vec![
                [ "User-agent".to_string(), "Mozilla/1.1N".to_string() ],
                [ "Proxy-authorization".to_string(), "basic aGVsbG86d29ybGQ=".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "utf-8 path request".to_string(),
            tp: HttpParserType::Request,
            strict: false,
            raw: "GET /δ¶/δt/pope?q=1#narf HTTP/1.1\r\n\
                Host: github.com\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "q=1".to_string(),
            fragment: "narf".to_string(),
            request_path: "/δ¶/δt/pope".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/δ¶/δt/pope?q=1#narf".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 1,
            headers: vec![
                [ "Host".to_string(), "github.com".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "hostname underscore".to_string(),
            tp: HttpParserType::Request,
            strict: false,
            raw: "CONNECT home_0.netscape.com:443 HTTP/1.0\r\n\
                User-agent: Mozilla/1.1N\r\n\
                Proxy-authorization: basic aGVsbG86d29ybGQ=\r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            method: HttpMethod::Connect,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "home_0.netscape.com:443".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 2,
            upgrade: Some(String::new()),
            headers: vec![
                [ "User-agent".to_string(), "Mozilla/1.1N".to_string() ],
                [ "Proxy-authorization".to_string(), "basic aGVsbG86d29ybGQ=".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "eat CRLF between requests, no \"Connection: close\" header".to_string(),
            raw: "POST / HTTP/1.1\r\n\
                Host: www.example.com\r\n\
                Content-Type: application/x-www-form-urlencoded\r\n\
                Content-Length: 4\r\n\
                \r\n\
                q=42\r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push(b'/');
                v
            },
            num_headers: 3,
            headers: vec![
                [ "Host".to_string(), "www.example.com".to_string() ],
                [ "Content-Type".to_string(), "application/x-www-form-urlencoded".to_string() ],
                [ "Content-Length".to_string(), "4".to_string() ],
            ],
            body: "q=42".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "eat CRLF between requests even if \"Connection: close\" is set".to_string(),
            raw: "POST / HTTP/1.1\r\n\
                Host: www.example.com\r\n\
                Content-Type: application/x-www-form-urlencoded\r\n\
                Content-Length: 4\r\n\
                Connection: close\r\n\
                \r\n\
                q=42\r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Post,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push(b'/');
                v
            },
            num_headers: 4,
            headers: vec![
                [ "Host".to_string(), "www.example.com".to_string() ],
                [ "Content-Type".to_string(), "application/x-www-form-urlencoded".to_string() ],
                [ "Content-Length".to_string(), "4".to_string() ],
                [ "Connection".to_string(), "close".to_string() ],
            ],
            body: "q=42".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "PURGE request".to_string(),
            tp: HttpParserType::Request,
            raw: "PURGE /file.txt HTTP/1.1\r\n\
                Host: www.example.com\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Purge,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/file.txt".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "/file.txt".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 1,
            headers: vec![
                [ "Host".to_string(), "www.example.com".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "SEARCH request".to_string(),
            tp: HttpParserType::Request,
            raw: "SEARCH / HTTP/1.1\r\n\
                Host: www.example.com\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Search,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push(b'/');
                v
            },
            num_headers: 1,
            headers: vec![
                [ "Host".to_string(), "www.example.com".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "host:port and basic_auth".to_string(),
            tp: HttpParserType::Request,
            raw: "GET http://a%12:b!&*$@hypnotoad.org:1234/toto HTTP/1.1\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/toto".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                for b in "http://a%12:b!&*$@hypnotoad.org:1234/toto".as_bytes() {
                    v.push(*b);
                }
                v
            },
            host: "hypnotoad.org".to_string(),
            userinfo: "a%12:b!&*$".to_string(),
            port: 1234,
            num_headers: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "line folding in header value".to_string(),
            tp: HttpParserType::Request,
            raw: "GET / HTTP/1.1\r\n\
                Line1:    abc\n\tdef\n ghi\n\t\tjkl\n mno \n\t \tqrs\n\
                Line2: \t line2\t\n\
                Line3:\n line3\n\
                Line4: \n \n\
                Connection:\n close\n\
                \n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/".to_string(),
            request_url: {
                let mut v: Vec<u8> = Vec::new();
                v.push(b'/');
                v
            },
            num_headers: 5,
            headers: vec![
                [ "Line1".to_string(), "abc\tdef ghi\t\tjkl mno \t \tqrs".to_string() ],
                [ "Line2".to_string(), "line2\t".to_string() ],
                [ "Line3".to_string(), "line3".to_string() ],
                [ "Line4".to_string(), "".to_string() ],
                [ "Connection".to_string(), "close".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "multiple connection header values with folding".to_string(),
            tp: HttpParserType::Request,
            raw: "GET /demo HTTP/1.1\r\n\
                Host: example.com\r\n\
                Connection: Something,\r\n Upgrade, ,Keep-Alive\r\n\
                Sec-WebSocket-Key2: 12998 5 Y3 1  .P00\r\n\
                Sec-WebSocket-Protocol: sample\r\n\
                Upgrade: WebSocket\r\n\
                Sec-WebSocket-Key1: 4 @1  46546xW%0l 1 5\r\n\
                Origin: http://example.com\r\n\
                \r\n\
                Hot diggity dogg".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            method: HttpMethod::Get,
            query_string: "".to_string(),
            fragment: "".to_string(),
            request_path: "/demo".to_string(),
            request_url: {
                let mut v = Vec::new();
                for b in "/demo".as_bytes() {
                    v.push(*b);
                }
                v
            },
            num_headers: 7,
            upgrade: Some("Hot diggity dogg".to_string()),
            headers: vec![
                [ "Host".to_string(), "example.com".to_string() ],
                [ "Connection".to_string(), "Something, Upgrade, ,Keep-Alive".to_string() ],
                [ "Sec-WebSocket-Key2".to_string(), "12998 5 Y3 1  .P00".to_string() ],
                [ "Sec-WebSocket-Protocol".to_string(), "sample".to_string() ],
                [ "Upgrade".to_string(), "WebSocket".to_string() ],
                [ "Sec-WebSocket-Key1".to_string(), "4 @1  46546xW%0l 1 5".to_string() ],
                [ "Origin".to_string(), "http://example.com".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
    ];

    const GET_NO_HEADERS_NO_BODY : usize = 4;
    const GET_ONE_HEADER_NO_BODY : usize = 5;
    const GET_FUNKY_CONTENT_LENGTH : usize = 6;
    const POST_IDENTITY_BODY_WORLD : usize = 7;
    const POST_CHUNKED_ALL_YOUR_BASE : usize = 8;
    const TWO_CHUNKS_MULT_ZERO_END : usize = 9;
    const CHUNKED_W_TRAILING_HEADERS : usize = 10;
    const CHUNKED_W_BULLSHIT_AFTER_LENGTH : usize = 11;
    const QUERY_URL_WITH_QUESTION_MARK_GET : usize = 14;
    const PREFIX_NEWLINE_GET : usize = 15;
    const CONNECT_REQUEST : usize = 17;

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
