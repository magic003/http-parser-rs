extern crate http_parser;

use std::default::Default;

use http_parser::{HttpParser, HttpParserType, HttpErrno, HttpVersion};

mod helper;

#[test]
fn test_responses() {
    // RESPONSES
    let responses: [helper::Message; 22] = [
        helper::Message {
            name: String::from_str("google 301"),
            tp: HttpParserType::Response,
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
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 301,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("Moved Permanently".as_bytes());
                    v
            },
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
        helper::Message {
            name: String::from_str("no content-length response"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 200 OK\r\n\
                Date: Tue, 04 Aug 2009 07:59:32 GMT\r\n\
                Server: Apache\r\n\
                X-Powered-By: Servlet/2.5 JSP/2.1\r\n\
                Content-Type: text/xml; charset=utf-8\r\n\
                Connection: close\r\n\
                \r\n\
                <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
                <SOAP-ENV:Envelope xmlns:SOAP-ENV=\"http://schemas.xmlsoap.org/soap/envelope/\">\n\
                  <SOAP-ENV:Body>\n\
                    <SOAP-ENV:Fault>\n\
                      <faultcode>SOAP-ENV:Client</faultcode>\n\
                      <faultstring>Client Error</faultstring>\n\
                    </SOAP-ENV:Fault>\n\
                  </SOAP-ENV:Body>\n\
                </SOAP-ENV:Envelop>"),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("OK".as_bytes());
                    v
            },
            num_headers: 5,
            headers: vec![
                [ String::from_str("Date"), String::from_str("Tue, 04 Aug 2009 07:59:32 GMT") ],
                [ String::from_str("Server"), String::from_str("Apache") ],
                [ String::from_str("X-Powered-By"), String::from_str("Servlet/2.5 JSP/2.1") ],
                [ String::from_str("Content-Type"), String::from_str("text/xml; charset=utf-8") ],
                [ String::from_str("Connection"), String::from_str("close") ],
            ],
            body: String::from_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
                                    <SOAP-ENV:Envelope xmlns:SOAP-ENV=\"http://schemas.xmlsoap.org/soap/envelope/\">\n\
                                      <SOAP-ENV:Body>\n\
                                        <SOAP-ENV:Fault>\n\
                                          <faultcode>SOAP-ENV:Client</faultcode>\n\
                                          <faultstring>Client Error</faultstring>\n\
                                        </SOAP-ENV:Fault>\n\
                                      </SOAP-ENV:Body>\n\
                                    </SOAP-ENV:Envelop>"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("404 no headers no body"),
            tp: HttpParserType::Response,
            raw: String::from_str("HTTP/1.1 404 Not Found\r\n\r\n"),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 404,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("Not Found".as_bytes());
                    v
            },
            num_headers: 0,
            headers: vec![ ],
            body_size: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("301 no response phrase"),
            tp: HttpParserType::Response,
            raw: String::from_str("HTTP/1.1 301\r\n\r\n"),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 301,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("".as_bytes());
                    v
            },
            num_headers: 0,
            headers: vec![ ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("200 trailing space on chunked body"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 200 OK\r\n\
                Content-Type: text/plain\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                25  \r\n\
                This is the data in the first chunk\r\n\
                \r\n\
                1C\r\n\
                and this is the second one\r\n\
                \r\n\
                0  \r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("OK".as_bytes());
                    v
            },
            num_headers: 2,
            headers: vec![
                [ String::from_str("Content-Type"), String::from_str("text/plain") ],
                [ String::from_str("Transfer-Encoding"), String::from_str("chunked") ],
            ],
            body_size: 37+28,
            body: String::from_str("This is the data in the first chunk\r\n\
                                    and this is the second one\r\n"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("no carriage ret"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 200 OK\n\
                Content-Type: text/html; charset=utf-8\n\
                Connection: close\n\
                \n\
                these headers are from http://news.ycombinator.com/"),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("OK".as_bytes());
                    v
            },
            num_headers: 2,
            headers: vec![
                [ String::from_str("Content-Type"), String::from_str("text/html; charset=utf-8") ],
                [ String::from_str("Connection"), String::from_str("close") ],
            ],
            body: String::from_str("these headers are from http://news.ycombinator.com/"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("proxy connection"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 200 OK\r\n\
                Content-Type: text/html; charset=UTF-8\r\n\
                Content-Length: 11\r\n\
                Proxy-Connection: close\r\n\
                Date: Thu, 31 Dec 2009 20:55:48 +0000\r\n\
                \r\n\
                hello world"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("OK".as_bytes());
                    v
            },
            num_headers: 4,
            headers: vec![
                [ String::from_str("Content-Type"), String::from_str("text/html; charset=UTF-8") ],
                [ String::from_str("Content-Length"), String::from_str("11") ],
                [ String::from_str("Proxy-Connection"), String::from_str("close") ],
                [ String::from_str("Date"), String::from_str("Thu, 31 Dec 2009 20:55:48 +0000") ],
            ],
            body: String::from_str("hello world"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("underscore header key"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 200 OK\r\n\
                Server: DCLK-AdSvr\r\n\
                Content-Type: text/xml\r\n\
                Content-Length: 0\r\n\
                DCLK_imp: v7;x;114750856;0-0;0;17820020;0/0;21603567/21621457/1;;~okv=;dcmt=text/xml;;~cs=o\r\n\r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("OK".as_bytes());
                    v
            },
            num_headers: 4,
            headers: vec![
                [ String::from_str("Server"), String::from_str("DCLK-AdSvr") ],
                [ String::from_str("Content-Type"), String::from_str("text/xml") ],
                [ String::from_str("Content-Length"), String::from_str("0") ],
                [ String::from_str("DCLK_imp"), String::from_str("v7;x;114750856;0-0;0;17820020;0/0;21603567/21621457/1;;~okv=;dcmt=text/xml;;~cs=o") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("bonjourmadame.fr"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.0 301 Moved Permanently\r\n\
                Date: Thu, 03 Jun 2010 09:56:32 GMT\r\n\
                Server: Apache/2.2.3 (Red Hat)\r\n\
                Cache-Control: public\r\n\
                Pragma: \r\n\
                Location: http://www.bonjourmadame.fr/\r\n\
                Vary: Accept-Encoding\r\n\
                Content-Length: 0\r\n\
                Content-Type: text/html; charset=UTF-8\r\n\
                Connection: keep-alive\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            status_code: 301,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("Moved Permanently".as_bytes());
                    v
            },
            num_headers: 9,
            headers: vec![
                [ String::from_str("Date"), String::from_str("Thu, 03 Jun 2010 09:56:32 GMT") ],
                [ String::from_str("Server"), String::from_str("Apache/2.2.3 (Red Hat)") ],
                [ String::from_str("Cache-Control"), String::from_str("public") ],
                [ String::from_str("Pragma"), String::from_str("") ],
                [ String::from_str("Location"), String::from_str("http://www.bonjourmadame.fr/") ],
                [ String::from_str("Vary"), String::from_str("Accept-Encoding") ],
                [ String::from_str("Content-Length"), String::from_str("0") ],
                [ String::from_str("Content-Type"), String::from_str("text/html; charset=UTF-8") ],
                [ String::from_str("Connection"), String::from_str("keep-alive") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("field underscore"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 200 OK\r\n\
                Date: Tue, 28 Sep 2010 01:14:13 GMT\r\n\
                Server: Apache\r\n\
                Cache-Control: no-cache, must-revalidate\r\n\
                Expires: Mon, 26 Jul 1997 05:00:00 GMT\r\n\
                .et-Cookie: PlaxoCS=1274804622353690521; path=/; domain=.plaxo.com\r\n\
                Vary: Accept-Encoding\r\n\
                _eep-Alive: timeout=45\r\n\
                _onnection: Keep-Alive\r\n\
                Transfer-Encoding: chunked\r\n\
                Content-Type: text/html\r\n\
                Connection: close\r\n\
                \r\n\
                0\r\n\r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("OK".as_bytes());
                    v
            },
            num_headers: 11,
            headers: vec![
                [ String::from_str("Date"), String::from_str("Tue, 28 Sep 2010 01:14:13 GMT") ],
                [ String::from_str("Server"), String::from_str("Apache") ],
                [ String::from_str("Cache-Control"), String::from_str("no-cache, must-revalidate") ],
                [ String::from_str("Expires"), String::from_str("Mon, 26 Jul 1997 05:00:00 GMT") ],
                [ String::from_str(".et-Cookie"), String::from_str("PlaxoCS=1274804622353690521; path=/; domain=.plaxo.com") ],
                [ String::from_str("Vary"), String::from_str("Accept-Encoding") ],
                [ String::from_str("_eep-Alive"), String::from_str("timeout=45") ],
                [ String::from_str("_onnection"), String::from_str("Keep-Alive") ],
                [ String::from_str("Transfer-Encoding"), String::from_str("chunked") ],
                [ String::from_str("Content-Type"), String::from_str("text/html") ],
                [ String::from_str("Connection"), String::from_str("close") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("non-ASCII in status line"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 500 Oriëntatieprobleem\r\n\
                Date: Fri, 5 Nov 2010 23:07:12 GMT+2\r\n\
                Content-Length: 0\r\n\
                Connection: close\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 500,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("Oriëntatieprobleem".as_bytes());
                v
            },
            num_headers: 3,
            headers: vec![
                [ String::from_str("Date"), String::from_str("Fri, 5 Nov 2010 23:07:12 GMT+2") ],
                [ String::from_str("Content-Length"), String::from_str("0") ],
                [ String::from_str("Connection"), String::from_str("close") ],
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("http version 0.9"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/0.9 200 OK\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 0, minor: 9 },
            status_code: 200,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("OK".as_bytes());
                v
            },
            num_headers: 0,
            headers: vec![
            ],
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("neither content-length nor transfer-encoding response"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 200 OK\r\n\
                Content-Type: text/plain\r\n\
                \r\n\
                hello world"),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("OK".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Content-Type"), String::from_str("text/plain") ],
            ],
            body: String::from_str("hello world"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("HTTP/1.0 with keep-alive and EOF-terminated 200 status"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.0 200 OK\r\n\
                Connection: keep-alive\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 0 },
            status_code: 200,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("OK".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Connection"), String::from_str("keep-alive") ],
            ],
            body_size: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("HTTP/1.0 with keep-alive and a 204 status"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.0 204 No content\r\n\
                Connection: keep-alive\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            status_code: 204,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("No content".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Connection"), String::from_str("keep-alive") ],
            ],
            body_size: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("HTTP/1.1 with an EOF-terminated 200 status"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 200 OK\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("OK".as_bytes());
                v
            },
            num_headers: 0,
            headers: vec![
            ],
            body_size: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("HTTP/1.1 with a 204 status"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 204 No content\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 204,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("No content".as_bytes());
                v
            },
            num_headers: 0,
            headers: vec![
            ],
            body_size: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("HTTP/1.1 with a 204 status and keep-alive disabled"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 204 No content\r\n\
                Connection: close\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 204,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("No content".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Connection"), String::from_str("close") ],
            ],
            body_size: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("HTTP/1.1 with chunked encoding and a 200 response"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 200 OK\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                0\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("OK".as_bytes());
                v
            },
            num_headers: 1,
            headers: vec![
                [ String::from_str("Transfer-Encoding"), String::from_str("chunked") ],
            ],
            body_size: 0,
            body: String::from_str(""),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("field space"),
            tp: HttpParserType::Response,
            strict: false,
            raw: String::from_str(
                "HTTP/1.1 200 OK\r\n\
                Server: Microsoft-IIS/6.0\r\n\
                X-Powered-By: ASP.NET\r\n\
                en-US Content-Type: text/xml\r\n\
                Content-Type: text/xml\r\n\
                Content-Length: 16\r\n\
                Date: Fri, 23 Jul 2010 18:45:38 GMT\r\n\
                Connection: keep-alive\r\n\
                \r\n\
                <xml>hello</xml>"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("OK".as_bytes());
                v
            },
            num_headers: 7,
            headers: vec![
                [ String::from_str("Server"), String::from_str("Microsoft-IIS/6.0") ],
                [ String::from_str("X-Powered-By"), String::from_str("ASP.NET") ],
                [ String::from_str("en-US Content-Type"), String::from_str("text/xml") ],
                [ String::from_str("Content-Type"), String::from_str("text/xml") ],
                [ String::from_str("Content-Length"), String::from_str("16") ],
                [ String::from_str("Date"), String::from_str("Fri, 23 Jul 2010 18:45:38 GMT") ],
                [ String::from_str("Connection"), String::from_str("keep-alive") ],
            ],
            body: String::from_str("<xml>hello</xml>"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("amazon.com"),
            tp: HttpParserType::Response,
            strict: false,
            raw: String::from_str(
                "HTTP/1.1 301 MovedPermanently\r\n\
                Date: Wed, 15 May 2013 17:06:33 GMT\r\n\
                Server: Server\r\n\
                x-amz-id-1: 0GPHKXSJQ826RK7GZEB2\r\n\
                p3p: policyref=\"http://www.amazon.com/w3c/p3p.xml\",CP=\"CAO DSP LAW CUR ADM IVAo IVDo CONo OTPo OUR DELi PUBi OTRi BUS PHY ONL UNI PUR FIN COM NAV INT DEM CNT STA HEA PRE LOC GOV OTC \"\r\n\
                x-amz-id-2: STN69VZxIFSz9YJLbz1GDbxpbjG6Qjmmq5E3DxRhOUw+Et0p4hr7c/Q8qNcx4oAD\r\n\
                Location: http://www.amazon.com/Dan-Brown/e/B000AP9DSU/ref=s9_pop_gw_al1?_encoding=UTF8&refinementId=618073011&pf_rd_m=ATVPDKIKX0DER&pf_rd_s=center-2&pf_rd_r=0SHYY5BZXN3KR20BNFAY&pf_rd_t=101&pf_rd_p=1263340922&pf_rd_i=507846\r\n\
                Vary: Accept-Encoding,User-Agent\r\n\
                Content-Type: text/html; charset=ISO-8859-1\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                1\r\n\
                \n\r\n\
                0\r\n\
                \r\n"),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 301,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("MovedPermanently".as_bytes());
                v
            },
            num_headers: 9,
            headers: vec![
                [ String::from_str("Date"), String::from_str("Wed, 15 May 2013 17:06:33 GMT") ],
                [ String::from_str("Server"), String::from_str("Server") ],
                [ String::from_str("x-amz-id-1"), String::from_str("0GPHKXSJQ826RK7GZEB2") ],
                [ String::from_str("p3p"), String::from_str("policyref=\"http://www.amazon.com/w3c/p3p.xml\",CP=\"CAO DSP LAW CUR ADM IVAo IVDo CONo OTPo OUR DELi PUBi OTRi BUS PHY ONL UNI PUR FIN COM NAV INT DEM CNT STA HEA PRE LOC GOV OTC \"") ],
                [ String::from_str("x-amz-id-2"), String::from_str("STN69VZxIFSz9YJLbz1GDbxpbjG6Qjmmq5E3DxRhOUw+Et0p4hr7c/Q8qNcx4oAD") ],
                [ String::from_str("Location"), String::from_str("http://www.amazon.com/Dan-Brown/e/B000AP9DSU/ref=s9_pop_gw_al1?_encoding=UTF8&refinementId=618073011&pf_rd_m=ATVPDKIKX0DER&pf_rd_s=center-2&pf_rd_r=0SHYY5BZXN3KR20BNFAY&pf_rd_t=101&pf_rd_p=1263340922&pf_rd_i=507846") ],
                [ String::from_str("Vary"), String::from_str("Accept-Encoding,User-Agent") ],
                [ String::from_str("Content-Type"), String::from_str("text/html; charset=ISO-8859-1") ],
                [ String::from_str("Transfer-Encoding"), String::from_str("chunked") ],
            ],
            body: String::from_str("\n"),
            ..Default::default()
        },
        helper::Message {
            name: String::from_str("empty reason phrase after space"),
            tp: HttpParserType::Response,
            raw: String::from_str(
                "HTTP/1.1 200 \r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: 200,
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                v.push_all("".as_bytes());
                v
            },
            num_headers: 0,
            headers: vec![
            ],
            body: String::from_str(""),
            ..Default::default()
        },
    ];

    const NO_HEADERS_NO_BODY_404 : uint = 2;
    const NO_REASON_PHRASE : uint = 3;
    const TRAILING_SPACE_ON_CHUNKED_BODY : uint = 4;
    const NO_CARRIAGE_RET : uint = 5;
    const UNDERSCORE_HEADER_KEY : uint = 7;
    const BONJOUR_MADAME_FR : uint = 8;
    const NO_BODY_HTTP10_KA_204 : uint = 14;

    // End of RESPONSES
    for m in responses.iter() {
        helper::test_message(m);
    }

    for m in responses.iter() {
        helper::test_message_pause(m);
    }

    for r1 in responses.iter() {
        if !r1.should_keep_alive { continue; }
        for r2 in responses.iter() {
            if !r2.should_keep_alive { continue; }
            for r3 in responses.iter() {
                helper::test_multiple3(r1, r2, r3);
            }
        }
    }

    test_message_count_body(&responses[NO_HEADERS_NO_BODY_404]);
    test_message_count_body(&responses[TRAILING_SPACE_ON_CHUNKED_BODY]);

    // test very large chunked response
    {
        let large_chunked = helper::Message {
            name: String::from_str("large chunked"),
            tp: HttpParserType::Response,
            raw: create_large_chunked_message(31337,
                "HTTP/1.0 200 OK\r\n\
                Transfer-Encoding: chunked\r\n\
                Content-Type: text/plain\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            status_code: 200,
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    v.push_all("OK".as_bytes());
                    v
            },
            num_headers: 2,
            headers: vec![
                [ String::from_str("Transfer-Encoding"), String::from_str("chunked") ],
                [ String::from_str("Content-Type"), String::from_str("text/plain") ],
            ],
            body_size: 31337*1024,
            ..Default::default()
        };
        test_message_count_body(&large_chunked);
    }

    // response scan 1/2
    helper::test_scan(&responses[TRAILING_SPACE_ON_CHUNKED_BODY],
              &responses[NO_BODY_HTTP10_KA_204],
              &responses[NO_REASON_PHRASE]);

    // response scan 2/2
    helper::test_scan(&responses[BONJOUR_MADAME_FR], 
              &responses[UNDERSCORE_HEADER_KEY],
              &responses[NO_CARRIAGE_RET]);
}

fn test_message_count_body(msg: &helper::Message) {
    let mut hp = HttpParser::new(msg.tp);
    hp.strict = msg.strict;

    let mut cb = helper::CallbackCountBody{..Default::default()};
    cb.messages.push(helper::Message{..Default::default()});

    let mut read : usize = 0;
    let len : usize = msg.raw.len();
    let chunk : usize = 4024;

    let mut i : usize = 0;
    while i < len {
        let toread : usize = std::cmp::min(len-i, chunk);
        read = hp.execute(&mut cb, msg.raw.as_bytes().slice(i, i + toread));
        if read != toread {
            helper::print_error(hp.errno.unwrap(), msg.raw.as_bytes(), read);
            panic!();
        }

        i += chunk;
    }

    cb.currently_parsing_eof = true;
    read = hp.execute(&mut cb, &[]);
    if read != 0 {
        helper::print_error(hp.errno.unwrap(), msg.raw.as_bytes(), read);
        panic!();
    }

    assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", msg.name);
    helper::assert_eq_message(&cb.messages[0], msg);
}

fn create_large_chunked_message(body_size_in_kb: uint, headers: &str) -> String {
    let mut buf = String::from_str(headers);

    for _ in range(0, body_size_in_kb) {
        buf.push_str("400\r\n");
        for _ in range(0i, 1024i) {
            buf.push('C');
        }
        buf.push_str("\r\n");
    }

    buf.push_str("0\r\n\r\n");
    buf
}

