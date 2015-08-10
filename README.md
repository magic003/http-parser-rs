# The Rust HTTP Parser

The Rust HTTP Parser provides the functionality to parse both HTTP requests and responses. It is ported from [joyent/http-parser](https://github.com/joyent/http-parser) written in C.

## Usage

Please refer to the [documentation](http://magic003.github.io/http-parser-rs/doc/http_parser/).

## Status

The parser is ported as well as unit tests. For the moment, all the test cases are passed. Besides that, I didn't do other tests around it and it was not used by other project either. So it is ready to be played around but far away to be used in production. Also, the API is unstable and may be changed.
