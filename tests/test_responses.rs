extern crate http_parser;

use std::default::Default;

use http_parser::{HttpParser, HttpParserType, HttpVersion};

pub mod helper;

#[test]
fn test_responses() {
    // RESPONSES
    let responses: [helper::Message; 22] = [
        helper::Message {
            name: "google 301".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 301 Moved Permanently\r\n\
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
                </BODY></HTML>\r\n".to_string(),
            should_keep_alive: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(301),
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    for b in "Moved Permanently".as_bytes() {
                        v.push(*b);
                    }
                    v
            },
            headers: vec![
                [ "Location".to_string(), "http://www.google.com/".to_string() ],
                [ "Content-Type".to_string(), "text/html; charset=UTF-8".to_string() ],
                [ "Date".to_string(), "Sun, 26 Apr 2009 11:11:49 GMT".to_string() ],
                [ "Expires".to_string(), "Tue, 26 May 2009 11:11:49 GMT".to_string() ],
                [ "X-$PrototypeBI-Version".to_string(), "1.6.0.3".to_string() ],
                [ "Cache-Control".to_string(), "public, max-age=2592000".to_string() ],
                [ "Server".to_string(), "gws".to_string() ],
                [ "Content-Length".to_string(), "219 ".to_string() ],
            ],
            body: "<HTML><HEAD><meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\">\n\
                        <TITLE>301 Moved</TITLE></HEAD><BODY>\n\
                        <H1>301 Moved</H1>\n\
                        The document has moved\n\
                        <A HREF=\"http://www.google.com/\">here</A>.\r\n\
                        </BODY></HTML>\r\n".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "no content-length response".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 200 OK\r\n\
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
                </SOAP-ENV:Envelop>".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    for b in "OK".as_bytes() {
                        v.push(*b);
                    }
                    v
            },
            headers: vec![
                [ "Date".to_string(), "Tue, 04 Aug 2009 07:59:32 GMT".to_string() ],
                [ "Server".to_string(), "Apache".to_string() ],
                [ "X-Powered-By".to_string(), "Servlet/2.5 JSP/2.1".to_string() ],
                [ "Content-Type".to_string(), "text/xml; charset=utf-8".to_string() ],
                [ "Connection".to_string(), "close".to_string() ],
            ],
            body: "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
                        <SOAP-ENV:Envelope xmlns:SOAP-ENV=\"http://schemas.xmlsoap.org/soap/envelope/\">\n\
                          <SOAP-ENV:Body>\n\
                            <SOAP-ENV:Fault>\n\
                              <faultcode>SOAP-ENV:Client</faultcode>\n\
                              <faultstring>Client Error</faultstring>\n\
                            </SOAP-ENV:Fault>\n\
                          </SOAP-ENV:Body>\n\
                        </SOAP-ENV:Envelop>".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "404 no headers no body".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(404),
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    for b in "Not Found".as_bytes() {
                        v.push(*b);
                    }
                    v
            },
            headers: vec![ ],
            body_size: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "301 no response phrase".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 301\r\n\r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(301),
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    // FIXME no need to push?
                    for b in "".as_bytes() {
                         v.push(*b);
                    }
                    v
            },
            headers: vec![ ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "200 trailing space on chunked body".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 200 OK\r\n\
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
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    for b in "OK".as_bytes() {
                        v.push(*b);
                    }
                    v
            },
            headers: vec![
                [ "Content-Type".to_string(), "text/plain".to_string() ],
                [ "Transfer-Encoding".to_string(), "chunked".to_string() ],
            ],
            body_size: 37+28,
            body: "This is the data in the first chunk\r\n\
                    and this is the second one\r\n".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "no carriage ret".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 200 OK\n\
                Content-Type: text/html; charset=utf-8\n\
                Connection: close\n\
                \n\
                these headers are from http://news.ycombinator.com/".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    for b in "OK".as_bytes() {
                        v.push(*b);
                    }
                    v
            },
            headers: vec![
                [ "Content-Type".to_string(), "text/html; charset=utf-8".to_string() ],
                [ "Connection".to_string(), "close".to_string() ],
            ],
            body: "these headers are from http://news.ycombinator.com/".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "proxy connection".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 200 OK\r\n\
                Content-Type: text/html; charset=UTF-8\r\n\
                Content-Length: 11\r\n\
                Proxy-Connection: close\r\n\
                Date: Thu, 31 Dec 2009 20:55:48 +0000\r\n\
                \r\n\
                hello world".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    for b in "OK".as_bytes() {
                        v.push(*b);
                    }
                    v
            },
            headers: vec![
                [ "Content-Type".to_string(), "text/html; charset=UTF-8".to_string() ],
                [ "Content-Length".to_string(), "11".to_string() ],
                [ "Proxy-Connection".to_string(), "close".to_string() ],
                [ "Date".to_string(), "Thu, 31 Dec 2009 20:55:48 +0000".to_string() ],
            ],
            body: "hello world".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "underscore header key".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 200 OK\r\n\
                Server: DCLK-AdSvr\r\n\
                Content-Type: text/xml\r\n\
                Content-Length: 0\r\n\
                DCLK_imp: v7;x;114750856;0-0;0;17820020;0/0;21603567/21621457/1;;~okv=;dcmt=text/xml;;~cs=o\r\n\r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    for b in "OK".as_bytes() {
                        v.push(*b);
                    }
                    v
            },
            headers: vec![
                [ "Server".to_string(), "DCLK-AdSvr".to_string() ],
                [ "Content-Type".to_string(), "text/xml".to_string() ],
                [ "Content-Length".to_string(), "0".to_string() ],
                [ "DCLK_imp".to_string(), "v7;x;114750856;0-0;0;17820020;0/0;21603567/21621457/1;;~okv=;dcmt=text/xml;;~cs=o".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "bonjourmadame.fr".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.0 301 Moved Permanently\r\n\
                Date: Thu, 03 Jun 2010 09:56:32 GMT\r\n\
                Server: Apache/2.2.3 (Red Hat)\r\n\
                Cache-Control: public\r\n\
                Pragma: \r\n\
                Location: http://www.bonjourmadame.fr/\r\n\
                Vary: Accept-Encoding\r\n\
                Content-Length: 0\r\n\
                Content-Type: text/html; charset=UTF-8\r\n\
                Connection: keep-alive\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            status_code: Some(301),
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    for b in "Moved Permanently".as_bytes() {
                        v.push(*b);
                    }
                    v
            },
            headers: vec![
                [ "Date".to_string(), "Thu, 03 Jun 2010 09:56:32 GMT".to_string() ],
                [ "Server".to_string(), "Apache/2.2.3 (Red Hat)".to_string() ],
                [ "Cache-Control".to_string(), "public".to_string() ],
                [ "Pragma".to_string(), "".to_string() ],
                [ "Location".to_string(), "http://www.bonjourmadame.fr/".to_string() ],
                [ "Vary".to_string(), "Accept-Encoding".to_string() ],
                [ "Content-Length".to_string(), "0".to_string() ],
                [ "Content-Type".to_string(), "text/html; charset=UTF-8".to_string() ],
                [ "Connection".to_string(), "keep-alive".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "field underscore".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 200 OK\r\n\
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
                0\r\n\r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                    let mut v: Vec<u8> = Vec::new();
                    for b in "OK".as_bytes() {
                        v.push(*b);
                    }
                    v
            },
            headers: vec![
                [ "Date".to_string(), "Tue, 28 Sep 2010 01:14:13 GMT".to_string() ],
                [ "Server".to_string(), "Apache".to_string() ],
                [ "Cache-Control".to_string(), "no-cache, must-revalidate".to_string() ],
                [ "Expires".to_string(), "Mon, 26 Jul 1997 05:00:00 GMT".to_string() ],
                [ ".et-Cookie".to_string(), "PlaxoCS=1274804622353690521; path=/; domain=.plaxo.com".to_string() ],
                [ "Vary".to_string(), "Accept-Encoding".to_string() ],
                [ "_eep-Alive".to_string(), "timeout=45".to_string() ],
                [ "_onnection".to_string(), "Keep-Alive".to_string() ],
                [ "Transfer-Encoding".to_string(), "chunked".to_string() ],
                [ "Content-Type".to_string(), "text/html".to_string() ],
                [ "Connection".to_string(), "close".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "non-ASCII in status line".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 500 Oriëntatieprobleem\r\n\
                Date: Fri, 5 Nov 2010 23:07:12 GMT+2\r\n\
                Content-Length: 0\r\n\
                Connection: close\r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(500),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "Oriëntatieprobleem".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
                [ "Date".to_string(), "Fri, 5 Nov 2010 23:07:12 GMT+2".to_string() ],
                [ "Content-Length".to_string(), "0".to_string() ],
                [ "Connection".to_string(), "close".to_string() ],
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "http version 0.9".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/0.9 200 OK\r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 0, minor: 9 },
            status_code: Some(200),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "OK".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
            ],
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "neither content-length nor transfer-encoding response".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 200 OK\r\n\
                Content-Type: text/plain\r\n\
                \r\n\
                hello world".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "OK".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
                [ "Content-Type".to_string(), "text/plain".to_string() ],
            ],
            body: "hello world".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "HTTP/1.0 with keep-alive and EOF-terminated 200 status".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.0 200 OK\r\n\
                Connection: keep-alive\r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 0 },
            status_code: Some(200),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "OK".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
                [ "Connection".to_string(), "keep-alive".to_string() ],
            ],
            body_size: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "HTTP/1.0 with keep-alive and a 204 status".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.0 204 No content\r\n\
                Connection: keep-alive\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            status_code: Some(204),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "No content".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
                [ "Connection".to_string(), "keep-alive".to_string() ],
            ],
            body_size: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "HTTP/1.1 with an EOF-terminated 200 status".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 200 OK\r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "OK".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
            ],
            body_size: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "HTTP/1.1 with a 204 status".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 204 No content\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(204),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "No content".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
            ],
            body_size: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "HTTP/1.1 with a 204 status and keep-alive disabled".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 204 No content\r\n\
                Connection: close\r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(204),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "No content".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
                [ "Connection".to_string(), "close".to_string() ],
            ],
            body_size: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "HTTP/1.1 with chunked encoding and a 200 response".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 200 OK\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n\
                0\r\n\
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "OK".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
                [ "Transfer-Encoding".to_string(), "chunked".to_string() ],
            ],
            body_size: 0,
            body: "".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "field space".to_string(),
            tp: HttpParserType::Response,
            strict: false,
            raw: "HTTP/1.1 200 OK\r\n\
                Server: Microsoft-IIS/6.0\r\n\
                X-Powered-By: ASP.NET\r\n\
                en-US Content-Type: text/xml\r\n\
                Content-Type: text/xml\r\n\
                Content-Length: 16\r\n\
                Date: Fri, 23 Jul 2010 18:45:38 GMT\r\n\
                Connection: keep-alive\r\n\
                \r\n\
                <xml>hello</xml>".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "OK".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
                [ "Server".to_string(), "Microsoft-IIS/6.0".to_string() ],
                [ "X-Powered-By".to_string(), "ASP.NET".to_string() ],
                [ "en-US Content-Type".to_string(), "text/xml".to_string() ],
                [ "Content-Type".to_string(), "text/xml".to_string() ],
                [ "Content-Length".to_string(), "16".to_string() ],
                [ "Date".to_string(), "Fri, 23 Jul 2010 18:45:38 GMT".to_string() ],
                [ "Connection".to_string(), "keep-alive".to_string() ],
            ],
            body: "<xml>hello</xml>".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "amazon.com".to_string(),
            tp: HttpParserType::Response,
            strict: false,
            raw: "HTTP/1.1 301 MovedPermanently\r\n\
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
                \r\n".to_string(),
            should_keep_alive: true,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(301),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                for b in "MovedPermanently".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
                [ "Date".to_string(), "Wed, 15 May 2013 17:06:33 GMT".to_string() ],
                [ "Server".to_string(), "Server".to_string() ],
                [ "x-amz-id-1".to_string(), "0GPHKXSJQ826RK7GZEB2".to_string() ],
                [ "p3p".to_string(), "policyref=\"http://www.amazon.com/w3c/p3p.xml\",CP=\"CAO DSP LAW CUR ADM IVAo IVDo CONo OTPo OUR DELi PUBi OTRi BUS PHY ONL UNI PUR FIN COM NAV INT DEM CNT STA HEA PRE LOC GOV OTC \"".to_string() ],
                [ "x-amz-id-2".to_string(), "STN69VZxIFSz9YJLbz1GDbxpbjG6Qjmmq5E3DxRhOUw+Et0p4hr7c/Q8qNcx4oAD".to_string() ],
                [ "Location".to_string(), "http://www.amazon.com/Dan-Brown/e/B000AP9DSU/ref=s9_pop_gw_al1?_encoding=UTF8&refinementId=618073011&pf_rd_m=ATVPDKIKX0DER&pf_rd_s=center-2&pf_rd_r=0SHYY5BZXN3KR20BNFAY&pf_rd_t=101&pf_rd_p=1263340922&pf_rd_i=507846".to_string() ],
                [ "Vary".to_string(), "Accept-Encoding,User-Agent".to_string() ],
                [ "Content-Type".to_string(), "text/html; charset=ISO-8859-1".to_string() ],
                [ "Transfer-Encoding".to_string(), "chunked".to_string() ],
            ],
            body: "\n".to_string(),
            ..Default::default()
        },
        helper::Message {
            name: "empty reason phrase after space".to_string(),
            tp: HttpParserType::Response,
            raw: "HTTP/1.1 200 \r\n\
                \r\n".to_string(),
            should_keep_alive: false,
            message_complete_on_eof: true,
            http_version: HttpVersion { major: 1, minor: 1 },
            status_code: Some(200),
            response_status: {
                let mut v: Vec<u8> = Vec::new();
                // FIXME no need to push?
                for b in "".as_bytes() {
                    v.push(*b);
                }
                v
            },
            headers: vec![
            ],
            body: "".to_string(),
            ..Default::default()
        },
    ];

    const NO_HEADERS_NO_BODY_404 : usize = 2;
    const NO_REASON_PHRASE : usize = 3;
    const TRAILING_SPACE_ON_CHUNKED_BODY : usize = 4;
    const NO_CARRIAGE_RET : usize = 5;
    const UNDERSCORE_HEADER_KEY : usize = 7;
    const BONJOUR_MADAME_FR : usize = 8;
    const NO_BODY_HTTP10_KA_204 : usize = 14;

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
            name: "large chunked".to_string(),
            tp: HttpParserType::Response,
            raw: create_large_chunked_message(31337,
                "HTTP/1.0 200 OK\r\n\
                Transfer-Encoding: chunked\r\n\
                Content-Type: text/plain\r\n\
                \r\n"),
            should_keep_alive: false,
            message_complete_on_eof: false,
            http_version: HttpVersion { major: 1, minor: 0 },
            status_code: Some(200),
            response_status: {
                    let mut v = Vec::new();
                    for b in "OK".as_bytes() {
                        v.push(*b);
                    }
                    v
            },
            headers: vec![
                [ "Transfer-Encoding".to_string(), "chunked".to_string() ],
                [ "Content-Type".to_string(), "text/plain".to_string() ],
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

    let len : usize = msg.raw.len();
    let chunk : usize = 4024;

    let mut i : usize = 0;
    while i < len {
        let toread : usize = std::cmp::min(len-i, chunk);
        let read = hp.execute(&mut cb, &msg.raw.as_bytes()[i .. i + toread]);
        if read != toread {
            helper::print_error(hp.errno.unwrap(), msg.raw.as_bytes(), read);
            panic!();
        }

        i += chunk;
    }

    cb.currently_parsing_eof = true;
    let read = hp.execute(&mut cb, &[]);
    if read != 0 {
        helper::print_error(hp.errno.unwrap(), msg.raw.as_bytes(), read);
        panic!();
    }

    assert!(cb.num_messages == 1, "\n*** num_messages != 1 after testing '{}' ***\n\n", msg.name);
    helper::assert_eq_message(&cb.messages[0], msg);
}

fn create_large_chunked_message(body_size_in_kb: usize, headers: &str) -> String {
    let mut buf = headers.to_string();

    for _ in (0..body_size_in_kb) {
        buf.push_str("400\r\n");
        for _ in (0u32..1024u32) {
            buf.push('C');
        }
        buf.push_str("\r\n");
    }

    buf.push_str("0\r\n\r\n");
    buf
}

