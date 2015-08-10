//! # The Rust HTTP Parser
//!
//! The Rust HTTP Parser provides the functionality to parse both HTTP requests and responses.
//!
//! It is ported from [joyent/http-parser](https://github.com/joyent/http-parser) written in C.
//!
//! # Install
//!
//! Add `http_parser` to `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! http_parser = "0.0.1"
//! ```
//!
//! # Usage
//!
//! Define a callback struct:
//!
//! ```
//! # use http_parser::*;
//! struct Callback;
//!
//! impl HttpParserCallback for Callback {
//!     fn on_message_begin(&mut self, parser: &mut HttpParser) -> CallbackResult {
//!         println!("Message begin");
//!         Ok(ParseAction::None)
//!     }
//!
//!     // Override other functions as you wish
//! }
//! ```
//!
//! Create an instance of `HttpParser` for requests:
//!
//! ```
//! # use http_parser::*;
//! let mut parser = HttpParser::new(HttpParserType::Request);
//! ```
//!
//! Create an instance of `Callback` struct:
//!
//! ```
//! # use http_parser::*;
//! # struct Callback;
//! #
//! # impl HttpParserCallback for Callback {
//! #     fn on_message_begin(&mut self, parser: &mut HttpParser) -> CallbackResult {
//! #         println!("Message begin");
//! #         Ok(ParseAction::None)
//! #     }
//! #
//! #     // Override other functions as you wish
//! # }
//! let mut cb = Callback;
//! ```
//!
//! Execute the parser by providing a HTTP request:
//!
//! ```
//! # use http_parser::*;
//! # struct Callback;
//! #
//! # impl HttpParserCallback for Callback {
//! #     fn on_message_begin(&mut self, parser: &mut HttpParser) -> CallbackResult {
//! #         println!("Message begin");
//! #         Ok(ParseAction::None)
//! #     }
//! #
//! #     // Override other functions as you wish
//! # }
//! # let mut cb = Callback;
//! # let mut parser = HttpParser::new(HttpParserType::Request);
//! let line: &str = "GET / HTTP/1.1\r\n";
//! parser.execute(&mut cb, line.as_bytes());
//! ```

#![crate_name = "http_parser"]

pub use self::parser::{HttpParser, HttpParserType};
pub use self::http_version::HttpVersion;
pub use self::error::HttpErrno;
pub use self::http_method::HttpMethod;
pub use self::callback::{HttpParserCallback, CallbackResult, ParseAction};

mod parser;
mod http_version;
mod error;
mod state;
mod flags;
mod http_method;
mod callback;

