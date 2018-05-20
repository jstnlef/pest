// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate pest;
extern crate pest_grammars;

use pest_grammars::http::{HttpParser, Rule};

#[test]
fn http_version() {
    parses_to! {
        parser: HttpParser,
        input: "HTTP/1.1",
        rule: Rule::http_version,
        tokens: [
            http_version(0, 8)
        ]
    };

    parses_to! {
        parser: HttpParser,
        input: "HTTP/2.13",
        rule: Rule::http_version,
        tokens: [
            http_version(0, 9)
        ]
    };
}

#[test]
fn method() {
    parses_to! {
        parser: HttpParser,
        input: "GET",
        rule: Rule::method,
        tokens: [
            method(0, 3)
        ]
    };

    parses_to! {
        parser: HttpParser,
        input: "POST",
        rule: Rule::method,
        tokens: [
            method(0, 4)
        ]
    };

    parses_to! {
        parser: HttpParser,
        input: "any",
        rule: Rule::method,
        tokens: [
            method(0, 3)
        ]
    };
}

